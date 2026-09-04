# Should `Op::Read` carry variant-level `#[non_exhaustive]`, and does it have to land in the same release as the `to` field?

Short answer up front so the rest reads as evidence: **yes to the attribute, and
yes it has to land now — but the deadline is real for a narrower reason than the
audit gives, and the strongest case against it is not the one the audit
anticipated.** The `to` field landed at
`crates/happenstance-testkit/src/model.rs` in the remediation wave that produced
this brief. That change already spent the break. Applying the attribute later
spends it a second time, on a population that will by then exist.

The counter-argument this brief has to beat is the constitution's own
[RS-13-5](../../../standards/rust/13-sealing-and-exhaustiveness.md), which says
in terms: *do not put `#[non_exhaustive]` on an enum designed not to grow.* It is
answered below, and the answer turns on a distinction the atom itself draws and
the audit did not: **`Op` the enum is designed not to grow; `Op::Read` the
variant is designed to.**

---

## Why this is owed

**The audit entry.** `references/evaluation/review-pre-publication-2026-09-03.md`,
finding **L2-01**, which routes this half of the finding away from the rule-set
owner:

> The `#[non_exhaustive]` question belongs to whoever owns the testkit's public
> surface at phase 12 (`RUNBOOK.md:163` — *"`cargo-semver-checks` reporting
> against a registry baseline"*), because that is the step that will notice it
> and the last one that can act cheaply.

**And it names the coupling that makes the routing awkward:**

> Applying the attribute is itself breaking, for the same reason. Both are free
> exactly once, at `0.2.0`.

The generator half has now been implemented and the attribute half has not, so
the two are no longer "both free once" — one of them has been spent. This brief
exists because that asymmetry is a decision nobody has taken, and the window in
which it is cheap closes at the same release the audit named.

---

## What is true today

### 1. `Op` is a `pub enum` on a published crate, with no `#[non_exhaustive]` at either level

`crates/happenstance-testkit/src/model.rs`, the declaration:

```rust
#[derive(Debug, Clone)]
pub enum Op {
```

Reachable as `happenstance_testkit::model::Op` whenever the `proptest` feature is
on. The module is gated `#[cfg(all(feature = "proptest", not(target_arch =
"wasm32")))]` (`crates/happenstance-testkit/src/lib.rs`), which narrows *who* is
exposed and not *whether*: a feature is a public surface, and `--all-features`
docs.rs builds render this type.

The crate is on the registry at `0.2.0-alpha.1`, published 2026-08-16
(`msrv-premise.md` in this directory has the queried table). So this is not a
pre-publication type in the sense that nothing downstream could exist.

### 2. The `to` field has landed, and it was breaking

`Op::Read` was:

```rust
Read { query: Query, from: Anchor, backwards: bool, limit: Option<usize> }
```

and is now:

```rust
Read { query: Query, from: Anchor, to: Anchor, backwards: bool, limit: Option<usize> }
```

Adding a field to a struct-form variant of a `pub enum` with no
`#[non_exhaustive]` breaks two things downstream: every `match` written with a
struct pattern that does not end in `..`, and every construction. Both are
compile errors, which is the good half of the news — nothing about this break is
silent.

The change is in `CHANGELOG.md` under `[Unreleased] / ### Changed`, marked
BREAKING, with the migration (`to: Anchor::Unset`) spelled out.

### 3. The variant is documented as designed to grow, and by the crate itself

`Op::Read`'s own `limit` field carries the note:

> A truncation, if any. Never zero — `ReadOptions::limit(0)` is `None` today and
> VT-28 owns changing that, in phase 4.

That is a second growth already named in the source: when the generator learns to
propose a zero budget, the field's *domain* changes; and `LimitZeroIsUnlimitedStore`
sits in `MODEL_COVERAGE` marked `Agreed` today for exactly that reason, which is a
standing invitation to widen the field or add a sibling. `ReadOptions` itself is
`#[non_exhaustive]` upstream in `happenstance-core` and grew `to` at phase 4 —
which is *how this hole arrived*: the options struct absorbed a new field without
a break, and the model's mirror of it could not.

The pattern is therefore not speculative. `Op::Read` is a mirror of a
`#[non_exhaustive]` upstream struct, and mirrors of growing things grow.

### 4. The crate already owns the argument for variant-level, and states it in terms

`crates/happenstance-testkit/src/lib.rs`, in the `Query::Items` compile test:

> Reading the variant from outside the crate still works, which is what
> variant-level `#[non_exhaustive]` buys over enum-level: matching is allowed and
> construction is not.

and, measured on 1.97.1 in the same comment, the sharp edge that goes with it:
`Query::Items(..)` — the *tuple* pattern — becomes `error[E0603]: tuple variant
'Items' is private` downstream, because a tuple pattern resolves through the
variant's constructor. Braces reach the fields directly and compile.
`Op::Read` is a struct-form variant, so that particular edge does not apply to
it; the relevant consequence is the other one, that downstream construction
becomes impossible.

**That consequence is the crux and it is not free.** See Option A's costs.

### 5. RS-13-5 is not violated by the *absence* at enum level, and the audit says so

`standards/rust/13-sealing-and-exhaustiveness.md`, RS-13-5:

> Do not put `#[non_exhaustive]` on an enum designed not to grow. … It costs
> downstream a `_ =>` arm forever, so the compiler permanently stops reporting
> the variant they forgot.

`Op`'s own documentation says the three variants *are* the whole port surface —
an unconditional append, a conditional one, and a read — and explains why there
is deliberately no fourth. That is a correct application of RS-13-5, and nothing
here proposes changing it. The question is entirely about the **variant**, where
RS-13-5's mechanism does not apply: variant-level `#[non_exhaustive]` costs a
`..` in a struct pattern, not a `_ =>` arm, and it does not stop the compiler
reporting anything a consumer forgot — matching is still exhaustive over the
enum.

### 6. Nothing in this workspace constructs an `Op`

Grepped: `Op::Read {` and `Op::Append {` appear only inside `model.rs` itself (the
generator's `prop_map` arms) and in the failure renderer's `Debug` output. The
example crate that exists to stand in for a stranger,
`examples/outside-projection-adapter/`, is a *projection* adapter and does not
depend on the `proptest` feature at all. **So this repository cannot observe the
cost of Option A**, and that is flagged rather than glossed: the argument below
is about a consumer the tree has no instrument for.

---

## Options

### Option A — apply variant-level `#[non_exhaustive]` to `Op::Read`, in the same release as `to`

```rust
pub enum Op {
    Append { events: Vec<Event> },
    AppendConditional { events: Vec<Event>, query: Query, anchor: Anchor },
    #[non_exhaustive]
    Read { query: Query, from: Anchor, to: Anchor, backwards: bool, limit: Option<usize> },
}
```

- **Costs a caller:** they may no longer *construct* an `Op::Read`, ever, from
  outside the crate. Matching still works with a trailing `..`. This is the real
  price, and it is larger than the audit's framing suggests, because
  construction is not obviously a thing nobody wants: a consumer who wants to
  replay a *specific* failing sequence — the exact use the rule's own panic
  message invites, since it prints the minimised `Vec<Op>` — would have to
  reconstruct it, and cannot. Nothing today exposes a constructor to replace
  that.
- **Costs an adapter author:** nothing. An adapter invokes
  `event_store_model_conformance!` and never names `Op`.
- **Semver:** breaking, and it lands inside a break already being taken, so the
  *marginal* cost is zero. Every later field addition becomes non-breaking.

### Option B — apply it later, at phase 12's `cargo-semver-checks` step

- **Costs a caller:** a second breaking release of `happenstance-testkit`, after
  `0.2.0` has created the adapter population the crate's own README says to pin
  exactly against. The break is a compile error rather than a silent change, so
  it is survivable — but the README's advice (*"pin this crate exactly"*) means
  the population feels it on a deliberate upgrade rather than on `cargo update`,
  which softens the cost by roughly the amount it softens every other break here.
- **Costs an adapter author:** nothing, again.
- **Semver:** breaking, a second time, for the same reason as the first.

### Option C — never apply it; accept that every future field on `Op::Read` is a break

- **Costs a caller:** nothing today, and one break per future field. There are at
  least two plausible future fields on the table — VT-28's zero budget, and
  whatever the next `ReadOptions` growth is, given `ReadOptions` is
  `#[non_exhaustive]` precisely because it expects to grow.
- **Costs an adapter author:** nothing.
- **Semver:** honest and expensive. It also makes `Op` inconsistent with
  `ReadOptions`, the type it mirrors, which is the kind of inconsistency that
  gets discovered by the person paying for it.

### Option D — apply it, and add a builder or constructor for `Op::Read` in the same change

Option A plus a way to construct one from outside — `Op::read(query)` returning a
value with defaults, plus `with_from`/`with_to`/… or a small `ReadOp` builder.

- **Costs a caller:** nothing they had; gives back the ability Option A removes,
  in a form that is itself additive forever.
- **Costs an adapter author:** nothing.
- **Semver:** the same single break as Option A, plus new public API — which is
  new surface to keep, document under the crate's rustdoc obligations, and get
  right on the first try.

---

## Recommendation

**Option A, and D only if someone actually wants to construct an `Op`.**

The reasoning is arithmetic rather than aesthetic. The break has been taken. A
`#[non_exhaustive]` applied inside it costs the marginal consumer nothing beyond
what they are already paying, and it is the last release at which that is true —
`0.2.0` is the release that creates the adapter population, which is the same
argument the rule added in this wave was landed on and it is not weaker here.
Option B pays the same price twice for no benefit; the only thing B buys is
deferral to an owner who will have strictly less freedom than the owner who has
it today.

**The strongest objection, stated in its own words.** *"Option A removes an
ability nobody has asked to remove, to solve a problem nobody has had. The
crate's own failure message prints a minimised `Vec<Op>` and invites the reader
to think about that sequence; forbidding them to write it down is a regression in
the thing the model family is for. And RS-13-5 exists because
`#[non_exhaustive]` is applied reflexively, for consistency, by people who have
not costed it — which is precisely what 'the break is already being taken' is a
licence to do."*

That objection is right about the mechanism and wrong about the ledger, on one
fact: **the ability it defends does not exist in usable form today either.**
Constructing an `Op::Read` from outside requires constructing an `Anchor`, which
is a public C-like enum and fine, and a `Query`, which is not — `Query::Items` is
already `#[non_exhaustive]` and must be built through `Query::from_items`. So a
consumer replaying a printed sequence is already doing translation work, and
`Op::Read`'s constructor is not the thing standing between them and it. If that
consumer materialises, Option D is available *additively* at any time, which is
the asymmetry that settles it: A→D is free later, and B→A is not.

The objection does land in one place, and it should be honoured: **do not apply
the attribute to `Op::Append` or `Op::AppendConditional`.** Those two are not
mirrors of a growing upstream type, `Op`'s own documentation argues they are
complete, and applying it to them would be exactly the reflexive consistency
RS-13-5 forbids. The recommendation is one attribute on one variant, and the
asymmetry between the three variants is itself the evidence that it was costed.

**One thing that would flip this.** If `happenstance-testkit`'s public-surface
owner intends `Op` to become a sealed, opaque type behind a builder API at phase
12 — rather than a `pub enum` at all — then Option A is wasted motion and the
right move is to do that instead, once. This brief has no evidence either way and
does not assume.

---

## Cost of delay

- **The window is one release wide.** `0.2.0` is the release that creates the
  population the testkit's README tells to pin exactly. Before it, this costs
  nobody a red build; after it, it costs every adapter one deliberate upgrade.
  That is the same clock the `to` field was landed on, and it has already
  started.
- **Delay does not preserve optionality, it spends it.** Option A after `0.2.0`
  *is* Option B, at a strictly higher price. There is no version of waiting that
  makes the attribute cheaper.
- **A second field is already named in the source.** `Op::Read`'s `limit` doc
  points at VT-28's phase for a change to the zero case, and `LimitZeroIsUnlimitedStore`
  sits in `MODEL_COVERAGE` as an uncaught defect waiting on exactly that. If that
  work lands before the attribute does, it pays for a third break.
- **Nothing is lost by deciding late in the wave rather than early**, provided it
  is decided before the release. The change is one attribute and one line of
  rustdoc.

---

## What this does not settle

- **Whether `Op` should be an enum at all.** Named above as the one thing that
  would flip the recommendation; no evidence is offered here.
- **Whether a construction path for `Op::Read` is wanted.** Option D is described
  and not recommended, because no consumer has asked and this repository has no
  instrument that would notice one.
- **The other half of L2-01**, the generator hole, which is implemented rather
  than deferred and is described in `CHANGELOG.md` under `[Unreleased] /
  ### Changed`.
- **Whether the model family's rule needs a clause.** That is
  `.kb/open-questions/model-family-rule-has-no-clause.md`'s question, and the
  `to` change makes that atom's enumeration of what the rule composes incomplete
  — it lists ES-8, ES-9, ES-11, ES-14, ES-15, ES-18 and ES-25, and ES-16 now
  belongs in it. That correction is the atom owner's and is recorded here only so
  it is not lost.
