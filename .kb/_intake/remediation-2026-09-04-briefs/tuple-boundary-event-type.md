# Does a tuple `Boundary` admit members over different domain enums — and what is `Boundary::Event` then — and what is this crate's sealed-trait evolution rule?

Decision record: **B-4-B-5-tuple-boundary**. Audit entry:
`references/evaluation/review-pre-publication-2026-09-03.md:660-696`.

Two questions, one owner, per the entry's own Routing field (`:695`). The second is
separable and free; the first has a deadline.

---

## Why this is owed

**1. The constraint exists, is permanent after `0.2.0`, and is written down nowhere.**

`crates/happenstance/src/composition.rs:29-31` (and identically `:39-41`) binds every
tuple member after the first to the first member's event type:

```rust
            $($rest: $crate::Boundary<
                Event = <$first as $crate::Boundary>::Event,
            >,)+
```

The module's only stated limit is the arity ceiling — `composition.rs:8-10`:

```rust
//! The ceiling is **8**. rustdoc renders one impl block per arity, and sixteen
//! of them buries [`Boundary`](crate::Boundary)'s page; above eight, compose
//! tuples of tuples.
```

and the rendered arity-2 page says the same thing again, `composition.rs:105-106`:

```rust
    /// this design is being measured on. The arity ceiling is 8, and that is a
    /// *rendering* decision — rustdoc emits one impl block per arity.
```

The doc block runs from `composition.rs:96` to `:157` and never uses the word `Event`.
Its worked example composes two `Counter`s over one `Seat` enum (`:140-155`). So the
page documents in full the half that costs nothing and is a rendering decision, and
says nothing about the half that is permanent. A caller who writes `(seats, wallet)`
gets a where-clause mismatch on an associated type, raised from inside a macro
expansion, at their call site, with the composition page in front of them.

**2. Neither the governing decision nor the signed-off design contains the constraint —
or the associated type it hangs off.**

`references/adr/0020-fold-query-agreement.md:155-161` — the accepted record's own
Decision text — declares the trait as:

```rust
/// What the command loop consumes. Sealed: blanket-implemented for every
/// `DecisionModel` and macro-implemented for tuples of arity 2..=8.
pub trait Boundary: sealed::Sealed {
    fn query(&self) -> Result<Query, InvalidQuery>;
    fn absorb<C: Codec>(&mut self, event: &SequencedEvent, codec: &C)
        -> Result<(), CodecError>;
}
```

**No `type Event`.** The same phase's signed-off design says the same thing
(`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md:421-435`) —
and then, thirty lines later, specifies the command loop as
(`_design.md:524`):

```rust
    F: FnMut(&B) -> Result<alloc::vec::Vec<B::Event>, D>;
```

`B::Event` on a trait that, as written in the same file, has no `Event`. The two
signed-off artifacts contradict each other, and the implementation resolved the
contradiction silently, in commit `996853f`, by adding `type Event: DomainEvent` to
`Boundary` and a homogeneity bound to every tuple impl. **The constraint under
decision here is the unrecorded residue of that resolution**, not a decision anyone
took.

**3. The documented workaround re-introduces the exact defect ADR-0020 exists to
remove.**

The developer's two escapes are "merge two bounded contexts into one enum" or "drop to
`happenstance-core`". The first is not merely inconvenient. `boundary.rs:118` derives
each member's query from its **event type's whole declared set**:

```rust
        derive_query(M::Event::EVENT_TYPES, self.scope())
```

so merging two contexts' enums widens *every member's* query to the union of both
contexts' event types, and therefore widens the composed query and the append
condition the command loop builds from it (`command.rs:290`, `:309`). It also destroys
the exhaustiveness pressure `DecisionModel::apply` is designed around
(`domain.rs:181-185`: "Takes the event **by value**, so the implementor's `match` is
exhaustive over their own enum"). The cost is already visible inside a *single*
context in the canonical example — `examples/course-subscriptions/src/main.rs:306`:

```rust
            Enrolment::StudentSubscribed { .. } | Enrolment::StudentUnsubscribed { .. } => {}
```

ADR-0020's Context opens on precisely this axis: a query selecting a type the fold
ignores widens the boundary, and the fold naming a set the query did not is what
corrupts (`references/adr/0020-fold-query-agreement.md`, Context). The workaround the
missing documentation would have to recommend is the widening direction, applied
mechanically.

**4. The rationale defending the associated type states the semver rule backwards, and
that sentence is what will block the repair.** `boundary.rs:70-75`:

```rust
    /// The one domain enum every member of this boundary folds.
    ///
    /// Present from birth rather than added later: the trait is sealed and the
    /// crate publishes from it, and growing a sealed trait a required item is
    /// a breaking change no downstream crate could have prepared for.
    type Event: DomainEvent;
```

Sealing is what makes growing the trait *free*; the comment spends the seal's dividend
as if it were the seal's cost. A maintainer applying that sentence after `0.2.0`
declines an addition that is in fact non-breaking, and a reviewer citing it blocks the
relaxation this decision is about.

---

## What is true today

### The seal, and why the doc comment is wrong

`crates/happenstance/src/lib.rs:199` carries no `pub`:

```rust
mod sealed;
```

`crates/happenstance/src/sealed.rs:1-19`:

```rust
//! The seal behind [`Boundary`](crate::Boundary).
//!
//! `Sealed` is `pub` inside a private module, which is the only spelling that
//! is both nameable as a supertrait of a public trait and unnameable outside
//! this crate. `pub(crate)` would make `Boundary` "more private than" its own
//! bound; a public marker trait would be no seal at all.

use crate::DecisionModel;

/// Implemented for every decision model, and for tuples of them.
///
/// Nothing else can implement it, because nothing else can name it.
#[allow(
    unreachable_pub,
    reason = "pub in a private module is the sealed-supertrait spelling"
)]
pub trait Sealed {}

impl<M: DecisionModel> Sealed for M {}
```

`Boundary` is public (`lib.rs:219`, `pub use boundary::Boundary;`). Every `Boundary`
impl in existence is inside this crate: the blanket at `boundary.rs:112` and the seven
`impl_boundary_for_tuple!` expansions (`composition.rs:95`, `:161-166`).

**The correct statement of the rule.** Adding a required item to `Boundary` cannot
produce `error[E0046]` in any downstream crate, because no downstream crate contains an
`impl Boundary` for the new item to be missing from — a downstream type reaches
`Boundary` only through *this crate's* blanket impl, which supplies the new item along
with the rest. That is the exact inverse of the unsealed case the constitution already
writes down for `EventStore`, `standards/rust/40-public-surface-and-evolution.md:14-18`
(RS-40-1): "a new required method is `error[E0046]` in every adapter's crate, on a
*minor* bump" — true for a port a stranger implements, false for a trait a stranger
cannot name. Two residuals survive and are worth naming rather than eliding: a new
**method** name can create a method-resolution ambiguity at a downstream call site that
has another trait of the same method name in scope, and a new **associated type**
becomes nameable in signatures that must then keep it. Neither is `E0046`, and neither
is what the doc comment claims.

The audit's Routing field records that this rule is stated nowhere else — not in
`standards/rust/13-sealing-and-exhaustiveness.md` (whose rules are RS-13-1 … RS-13-5,
all about `#[non_exhaustive]` and private fields), not in `.kb/decisions/`, not in the
specification. Verified: `grep -n "sealed" standards/rust/13-*.md` returns only the
See-also line.

### The constraint is not load-bearing on the read path

Verified independently of the audit's refutation pass. `Boundary`'s two methods never
name `Self::Event` (`boundary.rs:95`, `:109`):

```rust
    fn query(&self) -> Result<Query, InvalidQuery>;
    fn absorb<C: Codec>(&mut self, event: &SequencedEvent, codec: &C) -> Result<(), CodecError>;
```

The tuple's `query` (`composition.rs:45-73`) folds the members' queries into a union
and never mentions `Event`. The tuple's `absorb` (`composition.rs:75-90`) delegates to
each member:

```rust
                $fv.absorb(event, codec)?;
                $($rv.absorb(event, codec)?;)+
```

and each member decodes through the blanket impl using *its own* `M::Event`
(`boundary.rs:146`, `decode_event::<M::Event, C>`), gated by *its own* derived query
(`boundary.rs:138`). So a heterogeneous tuple's read path is already correct; the union
query and the per-member nomination check are exactly what a two-context boundary
needs.

Every use of `<B as Boundary>::Event` is on the write path or the write path's test
mirror — `command.rs:229`, `:277`:

```rust
    F: FnMut(&B) -> Result<Vec<B::Event>, D>,
```

`command.rs:306` (`encode::<B::Event, C, S::Error, D>(&decided, codec)?`), and
`testing/mod.rs:183`, `:186` (`Decision<B::Event>` and the same closure). That is
where the constraint belongs, and it is not a verdict on its own, because
`composition.rs:43`:

```rust
            type Event = <$first as $crate::Boundary>::Event;
```

would then mean "a composed boundary may emit only the *first* member's events" — an
order-dependent meaning nothing in the API surface reveals.

### Nothing in the workspace exercises the heterogeneous case

Both worked examples compose models over one enum:
`examples/course-subscriptions/src/main.rs:433` composes `(Seats, StudentSeat)` — two
*different model types*, both `type Event = Enrolment` (`:297`, `:335`, `:373`) — and
`examples/transfers-on-sqlite/src/main.rs:44-50` composes two `Balance` models over one
ledger enum. Every tuple in `crates/happenstance/tests/composition.rs` is a tuple of
`Tally`. That is why the constraint has never been felt in-tree, and it is also the
strongest evidence for the status quo: nobody here has needed the thing being asked for.

### The fence shape that pins whichever answer is taken

`boundary.rs:34-65` already carries the shape, on the seal:

```rust
/// ```compile_fail
/// // compile_fail: boundary_cannot_be_implemented_outside_the_crate
/// use happenstance::{Boundary, Codec, CodecError, DomainEvent};
```

with `boundary.rs:26-32` explaining why the fence's *other* obligations are made
well-formed on purpose, so that "the **only** diagnostic left is the seal" (RS-62-1:
measure what the fence actually rejects). A sibling fence beside the arity-2 impl —
rejecting a heterogeneous tuple under Option A, or rejecting the wrong `Event` under
B/C/D — is non-breaking, is owed under every option, and is independent of the choice.
The crate-root doc budget does not constrain it: `MODULE_DOC_LINES` in
`crates/happenstance/tests/doc_budget.rs:16` applies to `lib.rs` only
(`doc_budget.rs:129`).

### Governing records, specification, semver state

- **Governing atom:** `.kb/decisions/0020-fold-query-agreement.md`
  (`kb-decision-0020`, `status: accepted`, `supersedes: null`, `superseded_by: null`).
  Long form: `references/adr/0020-fold-query-agreement.md` (411 lines).
  **This decision supersedes no accepted atom.** ADR-0020 decides that the derivation
  lives on a sealed `Boundary`, macro-implemented for tuples of arity 2..=8; it is
  *silent* on `Boundary::Event` and on member homogeneity, and none of the four options
  below falsifies a sentence it states. A new atom, `related: [kb-decision-0020]`, not a
  superseding one.
- **Specification:** silent, by the record's own design.
  `grep -n "Boundary\|DecisionModel\|compose" spec/SPECIFICATION.md` returns no clause
  about either type; the only `tuple` hits are `:795` (`EventId`) and `:1680`
  (`Query::Items`). `spec/SPECIFICATION.md:102-108` puts the typed layer outside the
  document's subject: "the project, not the crate of that name, which is the typed layer
  and one consumer of what follows". `references/adr/0020-fold-query-agreement.md:373-377`
  states the same thing as a scope exclusion: "**Anything below the port.**
  `DecisionModel`, `Boundary` and the derived `Query` live entirely in `happenstance`. No
  store can observe whether a query was derived or hand-written, so there is no
  conformance rule here that any adapter could fail". **So: no clause id, no maturity
  marker, and no conformance rule to move.** The nearest clause, VT-27
  (`spec/SPECIFICATION.md:1691-1737`, `[FROZEN]`), governs `Query`/`ReadOptions` in the
  contract crate and is undisturbed by every option here: the tuple issues one union
  query in one read, and VT-27's MUST binds an application that needs *different bounds
  per fragment*.
- **Published state:** `happenstance` is live. `CHANGELOG.md:306-313` —
  "`## [0.2.0-alpha.1] — 2026-08-16` … **The first published release, and it is a
  pre-release on purpose.** The API is expected to move until the stable `0.2.0`; only
  one alpha resolves at a time, and each is yanked when the next lands." (Note in
  passing: `RUNBOOK.md:158`'s status table still shows phase 7 as `not started` and the
  alpha as unreleased. The table is stale; that is a separate finding and not this
  decision's.)

---

## Options

### A. Keep homogeneity. Document it, and fence it.

Leave `composition.rs:29-31` and `:39-41` as they are. Add the requirement to
`composition.rs:8-10` and to the arity-2 doc block, name the workaround honestly
(including the query widening it causes), and add a `compile_fail` fence beside the
arity-2 impl in `boundary.rs:34-65`'s shape.

- **Costs a caller:** nothing changes; the diagnostic they already get acquires a page
  that explains it. A caller with two bounded contexts is told, in writing, to merge
  their enums — which widens both models' derived queries and both folds.
- **Costs an adapter author:** nothing. `Boundary` is below the port
  (`references/adr/0020-fold-query-agreement.md:373-377`).
- **Semver:** **none.** Documentation and a doctest.
- **Forecloses:** the *relaxation of the tuple impls* becomes breaking after `0.2.0`.
  That is all. Nothing else.

  > **Removed.** This bullet previously read "**Forecloses:** heterogeneous composition,
  > permanently, at `0.2.0` … Also forecloses reaching the DCB independence proposition
  > from the typed layer at all". Both claims are **false** and are struck, not rewritten
  > around. They were falsified by compilation against today's untouched tuple impls and
  > seal: `impl<E, B1, B2> Sealed`/`Boundary for Emitting<E, (B1, B2)>` is coherent beside
  > `impl<M: DecisionModel> Sealed for M`, takes heterogeneous members, and satisfies
  > `FnOnce(&B) -> Vec<B::Event>`. Option D is **orthogonal** to the tuple impls'
  > homogeneity bound — `Emitting` delegates to `B1` and `B2` separately and never needs
  > the pair to be a tuple `Boundary` — so D stacks on A unchanged and delivers `commit`
  > as well as read. A forecloses nothing D does not restore, and D is semver-additive at
  > any date. What A does defer is the *governance* price D carries (see Option D's
  > Supersession line), not the capability.

### B. Relax the where-clause. The tuple's `Event` stays the first member's.

Delete `<Event = <$first as $crate::Boundary>::Event>` from both where-clauses; keep
`composition.rs:43` as it stands. Verified minimal: `$first: Boundary` remains, so
`type Event` stays well-formed, and `query`/`absorb` compile and behave correctly
unchanged.

- **Costs a caller:** `(seats, wallet)` composes for reading. `decide` may emit only
  `Seats::Event` — so `(seats, wallet)` and `(wallet, seats)` differ in what may be
  emitted, with no diagnostic that explains why and no name in the signature that says
  "first".
- **Costs an adapter author:** nothing.
- **Semver:** **breaking** (the audit's Semver field, `:691`, is authoritative here).
  Mechanically the relaxation alone is a widening — every tuple that compiles today
  compiles after it, identically — and the residual break is inference: downstream
  generic code whose second member's event parameter was determined *by the equality
  bound* becomes under-determined (`E0282`/`E0283`). The larger break is what
  necessarily follows if `type Event` is later given a different meaning.
- **Forecloses:** the meaning "first member's events" becomes permanent at `0.2.0`.
  It does not foreclose D, which can be added on top additively.

### C. Split the trait: `Boundary` loses `Event`; a new trait carries it.

`Boundary` becomes `query` + `absorb` — exactly the trait
`references/adr/0020-fold-query-agreement.md:155-161` records — implemented for tuples
with no homogeneity bound. A second sealed trait (`Decides: Boundary { type Event:
DomainEvent; }`) carries the write-path type, is blanket-implemented for every
`DecisionModel`, is implemented for tuples **with** the homogeneity bound, and is what
`commit`/`commit_with`/`Given::when` bind.

- **Costs a caller:** a heterogeneous tuple composes for reading and for
  `given(...)`; passing it to `commit` is a trait-bound error naming a trait whose name
  can say what is missing ("this tuple does not declare what it emits"). A caller with
  a generic helper written as `fn f<B: Boundary>` that calls `commit` must change one
  bound. A caller naming `B::Event` in their own code must change the path.
- **Costs an adapter author:** nothing.
- **Semver:** **breaking.** Removing a public associated type, and changing `commit`'s
  bound.
- **Forecloses:** the compile-time link between what was read and what may be emitted.
  Today `commit(&store, seats, r, |_| Ok(vec![Payment::Charged]))` does not compile;
  after the split, whether it does depends on where `Decides` is implemented — and for
  the heterogeneous case it must be answered by something, later. It leaves the "what
  may a composed boundary emit" question **open** rather than answering it arbitrarily,
  which is the point of the split and also its incompleteness.

### D. Leave the tuple impls alone. Add a wrapper carrying a caller-chosen `Event`.

A new public type over a tuple of *unconstrained* `Boundary`s — sketch:
`impl<E: DomainEvent, B1: Boundary, B2: Boundary> Boundary for Emitting<E, (B1, B2)>`
with `type Event = E`, delegating `query` and `absorb` — plus its `Sealed` impl.
Coherent (a distinct self type), so the existing tuple impls are untouched.

- **Costs a caller:** ceremony, and a second composition syntax:
  `Emitting::<App, _>::new((seats, wallet))` beside the bare tuple. That is the same
  objection that killed the exported `compose!`
  (`references/adr/0020-fold-query-agreement.md:311`; `composition.rs:3-5`, "a macro in
  the caller's face is exactly the ceremony this design is measured on") wearing a
  different costume — with the mitigation that it is charged only in the case that
  today has no option at all.
- **Costs an adapter author:** nothing.
- **Semver:** **additive.** A new type and new impls; nothing existing changes.
- **Supersession:** **a superseding atom against `kb-decision-0020`.** This is the one
  option here that *does* contradict a sentence the accepted record states, and the
  brief previously denied it. `references/adr/0020-fold-query-agreement.md:313-315`:
  "**Exactly one path is licensed.** There is no infallible sibling of
  `Boundary::query`, no second way to obtain a `Tags` for `scope`, and no second
  composition syntax" — under `:143-144`, "none of them is optional, and none of them
  admits a second spelling", and with `:311` rejecting an exported `compose!` because it
  "puts a second syntax beside the one the tuple impls already provide". The enforced
  atom repeats it: composition is "zero caller-visible syntax"
  (`.kb/decisions/0020-fold-query-agreement.md:77`). `Emitting::<App, _>::new((seats,
  wallet))` **is** a second caller-visible composition syntax beside the bare tuple, so
  D is not `related:` — accepted atoms are immutable, so it costs a new atom that
  supersedes `kb-decision-0020`. **Semver-additive and governance-free are different
  things, and D is only the first.**
- **Forecloses:** nothing structurally, but it makes the wrapper a public type the
  crate then owns; adopting it now and taking B or C before `0.2.0` anyway would leave
  a type needing its own justification.

---

## Recommendation

**Option A — keep homogeneity, document it, and fence it — with the sealed-trait
doc-comment correction (`boundary.rs:72-74`) landing under it, and D named in the
record as the later restoration of the heterogeneous case at its own governance price.**

**This recommendation flipped.** The prior draft recommended **Option C**, "the split —
with A's documentation and fence landing under it, and D available afterwards as an
additive answer to the emit question", at medium confidence. Two findings flipped it,
both since verified above:

- **C's urgency rested on a foreclosure that does not exist.** Reason 4 below — the
  deadline asymmetry — bit only because Option A was said to foreclose heterogeneous
  composition permanently and to foreclose reaching the DCB independence proposition
  from the typed layer at all. Compilation against today's untouched tuple impls and
  seal falsified both; see the struck bullet under Option A. D stacks on A unchanged.
- **D is not governance-free, so "C now, D later" was never the cheap package it was
  priced as.** See Option D's Supersession line. That cuts against C harder than
  against A, because C is the option that *needs* D to finish the job it starts.

Confidence: **medium.** Unchanged, and for the same reason: the underlying product
premise — whether a DCB boundary may span two bounded contexts — is one the repository
has never tested, and A is the answer that declines to spend anything settling it.

Why A now beats C:

1. **C spends the one pre-`0.2.0` break and still cannot commit a heterogeneous
   tuple.** C keeps the homogeneity bound, moving it from `Boundary` to `Decides` for
   tuples (Option C's own text, and its Forecloses bullet), so
   `commit((seats, wallet), …)` remains a bound error after the break. C buys read-only
   composition plus trait hygiene; the *capability* still waits on D under C exactly as
   it does under A. Paying a break for half of a thing whose other half is unchanged by
   the payment is the wrong trade.
2. **Two of C's four reasons do not survive scrutiny.** Its reason 1 cited RS-40-1
   (`standards/rust/40-public-surface-and-evolution.md:12`), but RS-40-1's *why* is
   `error[E0046]` in downstream impls — impossible on a sealed trait, by this brief's
   own question-2 finding above — and the rule licenses *growing* a port with a new
   trait, not *removing* a public associated type, which is what C does. It also read
   `references/adr/0020-fold-query-agreement.md:155-161` as a recorded trait to move
   toward, when this brief elsewhere establishes that the record is **silent** on
   `Boundary::Event`: a sketch's omission is not a decision, and "moves toward its own
   accepted record" reads one into it.
3. **A is free in the currency the deadline is denominated in, and D's price is not
   deadline-bound.** A costs no semver and no supersession. D costs no semver ever, and
   its governance price — a superseding atom against `kb-decision-0020` — is the same
   before and after `0.2.0`. So the deadline no longer forces a choice between capability
   and cost; it only forces the question of whether the *relaxation of the tuple impls
   themselves* (B and C's mechanism) is worth a break, and nothing in the tree says it is.
4. **The status quo's own evidence still stands.** Nothing in the workspace exercises
   the heterogeneous case; every tuple in both worked examples and in every test is
   homogeneous. A is what "no user has asked for this yet" looks like when written down
   honestly instead of left implicit.

**The strongest argument against it, stated in its own words:**

> D is not the free additive escape hatch the recommendation leans on. The brief prices
> it "additive… free now, free forever… the one repair that survives the deadline", and
> recommends C "with D available afterwards". But `Emitting::<App, _>::new((seats,
> wallet))` is caller-visible composition syntax beside the tuple, which
> ADR-0020:313-315 licenses out of existence. The brief concedes the identity ("a second
> composition syntax… the objection that killed `compose!` in a different costume"), then
> asserts no option contradicts the record. Accepted atoms are immutable, so D costs a
> superseding atom, not `related:`. Semver-additive and governance-free differ, and only
> the second is what reason 4 ("the deadline is asymmetric") trades on. Strip D and C
> ships a split whose emit question has no cheap later answer — the Option A objection,
> unanswered.

Applied to A rather than to C, that argument lands as this: A leaves the heterogeneous
capability reachable **only** through a repair that requires superseding an accepted,
immutable atom, and nothing in the tree commits anyone to paying that. A decider who
believes a DCB boundary must be able to span two bounded contexts should weigh
superseding `kb-decision-0020` deliberately — now, on its own record — rather than
inheriting it as a debt A quietly creates. What that argument does **not** rescue is C,
because C leaves the capability behind the same atom and charges a `0.2.0` break on the
way.

The case *for* A, stated in the words it was first written in as an objection to C, and
kept because it is the substance of the flip:

> Removing `Boundary::Event` deletes the only compile-time link between what a boundary
> read and what a decision may emit. Today `commit(&store, seats, r, |_| Ok(vec![Payment::Charged]))`
> does not compile; after the split it is a question of where `Decides` lands, and the
> append condition — built from the query, never from the emitted events
> (`command.rs:309`) — will not catch the mistake at run time either. ADR-0020's entire
> thesis is structural prevention over detection: `query()` is not on `DecisionModel`
> "so there is nowhere to put a hand-maintained one", and the seal exists so no third
> implementation can appear (`references/adr/0020-fold-query-agreement.md:146-183`).
> Trading a structural guarantee for a composition **nothing in this workspace has ever
> needed** — every tuple in both worked examples and in every test is homogeneous — is
> the wrong direction for this codebase. The cheaper and more honest reading is that the
> constraint is *right*, that a consistency boundary names one application's event
> vocabulary (which is what `Enrolment` already is), and that the only real defect here
> is that nobody wrote it down. That is Option A, it costs nothing, and it is reversible
> in the one direction that matters — a later ADR can still relax, at the price of a
> minor bump, if a real user ever asks.

That argument is still not answered by anything in the tree, and taking A is agreeing
with it. **A must be taken deliberately, not by default.** Deciding nothing also lands
on A — but on the version of A that ships no documentation, no fence, and leaves
`boundary.rs:72-74`'s inverted rule on the page. The deliberate version ships all three.

**The second question — the sealed-trait evolution rule — is not contested and is
free.** `boundary.rs:72-74` is wrong in direction and should be replaced by the correct
statement above, under every option including A. Where the rule *lives* is a placement
choice (a rule in `standards/rust/13-sealing-and-exhaustiveness.md` beside RS-13-1…5, an
atom in `.kb/decisions/`, or both) and is not decided here.

---

## Cost of delay

**Mixed, and the mix is the reason to decide now rather than the reason not to.**

- **A** — free now, free forever. It is documentation; it never expires.
- **B and C** — free now, **permanent at `0.2.0`**. `happenstance` is published at
  `0.2.0-alpha.1` (`CHANGELOG.md:306`), whose release note explicitly buys the licence
  to move: "The API is expected to move until the stable `0.2.0`; only one alpha
  resolves at a time, and each is yanked when the next lands" (`CHANGELOG.md:308-313`).
  After phase 12 (`RUNBOOK.md:163`, "Publish `0.2.0`") that licence is spent and both
  cost a major-position bump.
- **D** — **semver**-free now and semver-free forever; **not** governance-free at any
  date. The words "free now, free forever" stood here for D without qualification and
  are struck: they conflated the two currencies. What is true is that D is additive, so
  it is the one repair that survives the deadline, *and* that it costs a superseding
  atom against `kb-decision-0020` whenever it is taken (Option D, Supersession). Both
  costs are constant across `0.2.0`, which is why D is not the thing to rush — not
  because it is free.
- **The doc-comment correction (`boundary.rs:72-74`)** — free now, free forever, and
  *appreciating in cost*: the longer it stands, the more likely it is cited. The audit
  names that as the concrete harm — "a maintainer applying `boundary.rs:72-74`'s rule
  after `0.2.0` declines a required addition to `Boundary` that is in fact free, and a
  reviewer citing it blocks the relaxation this entry is about"
  (`references/evaluation/review-pre-publication-2026-09-03.md:689`).

Deciding **nothing** is a decision for A, taken silently, with the wrong rationale still
on the page defending it.

---

## What this does not settle

- **What a composed boundary may emit when its members span more than one enum.** Under
  every option here, `decide` returns `Vec<E>` for one `E`; a caller who must emit into
  two contexts writes a union enum for the *write* path. C confines that union to the
  write path (the read-side queries stay narrow); A and B do not offer it at all. The
  general question — a sum type per arity, an `Either`, a caller-declared type — is out
  of scope and would be its own record.
- **The arity ceiling of 8.** Untouched by all four options; it remains the rendering
  decision `composition.rs:8-10` says it is.
- **`DecisionModel::Event`** (`domain.rs:170`). Nothing here proposes moving or removing
  it; it is the member's own enum and every option keeps it.
- **Where the sealed-trait evolution rule is written down.** The brief states the
  correct rule; it does not choose between `standards/rust/13-*` and a `.kb` atom, and
  the audit's Routing field routes both questions to one owner without merging them.
- **`kb-open-question-disjoint-boundaries-no-clause-001`**
  (`.kb/open-questions/disjoint-boundaries-have-no-clause.md`) — **left open, and
  untouched.** Its three ordered sub-questions are all about
  `spec/SPECIFICATION.md`: whether the independence proposition gets its own clause or
  widens the `[FROZEN]` ES-25, where such a clause would sit, and how
  `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1968-1997`) then reports. None of
  those is answered by anything above, and this decision adds no clause and no
  conformance rule — the typed layer is outside the specification's subject
  (`spec/SPECIFICATION.md:102-108`;
  `references/adr/0020-fold-query-agreement.md:373-377`). What C or B *would* change is
  the open question's motivation: today the store-level composition
  `k_disjoint_boundaries_admit_exactly_k_commits` proves is unreachable from the typed
  layer, so the missing clause has no typed-layer consumer to embarrass it. Relaxing the
  tuple gives it one — and so, additively and without a break, does **D**, which is why
  taking A does not put that consumer permanently out of reach.
- **Supersession.** **It depends on the option, and this bullet previously said it did
  not.** The claim "Nothing … no option here contradicts a sentence it states", and the
  blanket `supersedes: null`, are **removed**: they are false for **Option D**, falsified
  by `references/adr/0020-fold-query-agreement.md:313-315` ("Exactly one path is
  licensed … and no second composition syntax"), read under `:143-144` ("none of them is
  optional, and none of them admits a second spelling") and against the enforced atom's
  "zero caller-visible syntax" (`.kb/decisions/0020-fold-query-agreement.md:77`). D adds
  a second caller-visible composition syntax, and accepted atoms are immutable, so D
  requires an atom that **supersedes** `kb-decision-0020`.

  What survives: for **A, B and C** the record is genuinely silent — on `Boundary::Event`
  and on member homogeneity — so those three carry `related: [kb-decision-0020]` and
  `supersedes: null`. The recommended A therefore supersedes nothing *as taken*, and the
  supersession it defers is D's, not its own. The old claim that C "moves *toward*
  `references/adr/0020-fold-query-agreement.md:155-161`'s recorded trait" is also struck:
  that passage is a sketch in a record this brief establishes is silent on the associated
  type, and treating an omission as a recorded intent is what the claim did.
- **The stale `RUNBOOK.md:158` status row** (phase 7 `not started`, the alpha unreleased,
  against `CHANGELOG.md:306`'s published `0.2.0-alpha.1`). Noticed while grounding this;
  it belongs to whoever owns the runbook.

---

## Where the audit's own proposal is and is not supported

The entry's **Remediation** (`:693`) — a `compile_fail` doctest beside the arity-2
impl, in `boundary.rs:34-65`'s shape, "non-breaking and independent of the decision" —
is fully supported: the shape exists, the fence's design note at `boundary.rs:26-32`
already explains how to make it prove the thing it claims, and no doc budget constrains
the composition module.

The entry's **Semver** paragraph (`:691`) is supported in direction and slightly
imprecise in mechanism: the relaxation *alone* is a widening that every tuple compiling
today survives unchanged, and the break it carries is inference in downstream generic
code, not `E0046` or a signature change. The entry says as much itself — it calls the
refutation "evidence about where the constraint belongs; it is not a verdict" — and its
classification stands.

The entry's **Found** section is accurate in every particular checked here, with one
addition it does not make and which strengthens it: the associated type it examines is
absent from *both* the accepted decision record
(`references/adr/0020-fold-query-agreement.md:155-161`) and the signed-off design
(`_design.md:421-435`), while the same design's `commit` signature (`_design.md:524`)
depends on it. The constraint is not a decision that was made and left undocumented; it
is a contradiction between two signed-off artifacts that was resolved in code and never
recorded.

---

## Revision record

Two critiques were taken against the first draft. Neither was accepted as written in
full, and both changed the brief.

**1. The recommendation flipped: C → A.** The first draft recommended Option C (the
`Boundary`/`Decides` split) with D available afterwards. It now recommends **Option A**
— document the constraint, fence it, correct `boundary.rs:72-74` — with D named as the
later restoration at its own governance price.

What flipped it:

- **A falsified premise, removed rather than rewritten around.** Option A's Forecloses
  bullet claimed A "forecloses heterogeneous composition, permanently" and forecloses
  "reaching the DCB independence proposition from the typed layer at all". Both were
  falsified by compiling Option D against today's untouched tuple impls and seal:
  `impl<E, B1, B2> Sealed`/`Boundary for Emitting<E, (B1, B2)>` is coherent beside
  `impl<M: DecisionModel> Sealed for M`, takes heterogeneous members, and satisfies
  `FnOnce(&B) -> Vec<B::Event>`. D is orthogonal to the tuple impls' homogeneity bound
  (it delegates to `B1` and `B2` separately), so it stacks on A unchanged and delivers
  `commit` as well as read. The claims are struck in place, with the reason attached.
  This mattered because C's fourth reason — the asymmetric deadline — bit only through
  that foreclosure.
- **Two of C's four reasons did not survive.** Reason 1 cited RS-40-1, whose *why* is
  `error[E0046]` in downstream impls — impossible on a sealed trait by this brief's own
  question-2 finding — and which licenses *growing* a port with a new trait rather than
  *removing* a public associated type; and it read
  `references/adr/0020-fold-query-agreement.md:155-161` as a recorded trait to move
  toward, when the brief elsewhere calls that record *silent* on `Boundary::Event`.
  Both readings are withdrawn in the rewritten Recommendation.
- **C's incompleteness was priced too cheaply.** C keeps the homogeneity bound on
  `Decides` for tuples, so `commit((seats, wallet), …)` stays a bound error after the
  break; the capability waits on D under C exactly as under A.

**2. A second falsified premise: D is not governance-free.** The "Supersession" bullet
asserted "Nothing … no option here contradicts a sentence it states", with the new atom
carrying `supersedes: null` for every option. That is false for **D**:
`Emitting::<App, _>::new((seats, wallet))` is a second caller-visible composition syntax,
which `references/adr/0020-fold-query-agreement.md:313-315` licenses out of existence
("Exactly one path is licensed … and no second composition syntax") under `:143-144`
("none of them … admits a second spelling"), and which the enforced atom repeats as
"zero caller-visible syntax" (`.kb/decisions/0020-fold-query-agreement.md:77`). Accepted
atoms are immutable, so D costs a **superseding** atom, not `related:`. The blanket claim
is removed and replaced with a per-option statement; Option D gained a Supersession line;
and the Cost-of-delay bullet that priced D "free now, free forever" is struck for
conflating semver-additive with governance-free. Note that this critique argued *for* C
against a stripped D; it does not carry that conclusion here, because the same finding
cuts harder against C — C is the option that needs D to finish what it starts, while A
does not spend a break to arrive at the same waiting position.

**3. The surviving objection is stated verbatim under Recommendation.** Critique 2's
argument is quoted in its own words there as the strongest argument against the
recommendation, applied to A: A leaves the heterogeneous capability reachable only by
superseding an accepted, immutable atom, and nothing in the tree commits anyone to
paying that.

**Unchanged:** the four options' mechanics, the "What is true today" findings, the
second question (the sealed-trait evolution rule, uncontested and free under every
option), the Cost-of-delay entries for A, B, C and the doc comment, and the audit
reconciliation. Confidence stays **medium**.
