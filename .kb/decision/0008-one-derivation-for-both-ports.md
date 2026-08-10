---
id: adr-0008-one-derivation-for-both-ports
title: "ADR-0008: One derivation scheme, both ports, and what a provided body owes"
kind: decision
status: accepted
authority_tier: decision
summary: >-
  One derivation scheme covers `EventStore` and `ProjectionStore` in a single decision
  (PS-35), and a provided body must type-check under both flavours' bounds at once
  because `trait_variant` clones it into the variant. The scheme is shared; the
  provided-method budget is not.
depends_on:
  - adr-0001-async-port-flavours
related: []
source_paths:
  - docs/architecture/SPECIFICATION.md
  - crates/happenstance-core/src/store.rs
last_reviewed: 2026-08-09
adr_id: ADR-0008
phase: 1
supersedes: []
superseded_by: null
---

# ADR-0008: One derivation scheme, both ports, and what a provided body owes

- **Status:** accepted
- **Date:** 2026-08-06
- **Amends:** [ADR-0001](0001-async-port-flavours.md), whose `provisional` marker
  this decision lifts

## Context

[ADR-0001](0001-async-port-flavours.md) chose `trait_variant` over two
hand-written traits, and argued it for `EventStore` alone. Three things have
been true ever since and none of them were written down.

**`ProjectionStore` carries the identical construction and appears in no ADR.**
`projection.rs:70` is `#[trait_variant::make(SendProjectionStore: Send)]`, copied
from `store.rs:92`, decided by nobody. `SPECIFICATION.md`'s PS-35 says the
derivation decision must cover both ports in one document precisely because an
application holding a `!Send` projection store beside a `Send` event store is
ordinary rather than exotic, so the two decisions compose at every call site.

**A provided body is *cloned* into the variant, so one body must type-check under
both flavours' bounds simultaneously.** `variant.rs:161-168` copies the `default`
block while setting `asyncness: None` and does not rewrite it. Nobody had stated
the obligation that follows, and ES-4 and PS-37 now do.

**ADR-0001's evidence base was a `cargo check`.** Its own banner says it: *no
`!Send` implementation of these ports exists anywhere, not even a reference one*.
A compile of a trait is not a compile of an implementation, and the bare flavour
— the entire justification for the design — had zero implementers.

What is *not* open: whether to derive at all. ES-1 is `[FROZEN]` on "the `Send`
flavour MUST be derived by `#[trait_variant::make(…)]` rather than hand-written",
and this ADR does not reopen it. The queue phrased this ADR's question as a
choice between derived and hand-written; against the specification that framing
is stale, and the specification wins. What this ADR settles is whether *one*
scheme can serve *both* ports, and what it costs each of them.

## Decision

**One scheme, both ports, derived.** `ProjectionStore` keeps
`#[trait_variant::make(SendProjectionStore: Send)]` and is now covered by the
same reasoning as `EventStore` rather than by inheritance from a file it was
copied out of.

Three rules follow, and they are the part worth quoting:

1. **A provided method is hand-desugared to `-> impl Future`, never `async fn`.**
   The `async fn` spelling fails with `error[E0728]: await is only allowed inside
   async functions and blocks`, at the *port*, when the cloned body loses its
   asyncness.
2. **Any provided body holding `&self` across an `await` takes `where Self: Sync`
   at the point of use.** Never in the attribute: `trait_variant` appends the
   attribute's whole bound list to `read`'s stream as well, so
   `make(SendEventStore: Send + Sync)` rejects any adapter whose stream hides a
   `Cell` or an `Rc`.
3. **One body, both flavours, checked at the port.** A provided method that
   compiles against `EventStore` and not against `SendEventStore` is a compile
   error in `happenstance-core`, discovered by whoever adds the first defaulted
   method rather than by whoever designed the port.

### The evidence, compiled

Every claim below was reproduced against `trait-variant 0.1.3` on rustc 1.97.1.
This ADR quotes compilations rather than arguments because the two prior planning
documents both reached headline conclusions about this exact attribute by
argument, and both were refuted by compiling two lines
(`PRESSURE-TEST.md:32-54`).

**The provided body works, and the derived flavour's future really is `Send`.**
A `head` written in the hand-desugared form, whose body genuinely holds `&self`
across an await, compiles under the attribute. `-Zunpretty=expanded` shows three
copies: the bare trait's default, the derived trait's default with `+ Send`
appended, and a blanket-impl *override* delegating
`<Self as SendEventStore>::head(self)`. So a provided body is a fallback for the
bare flavour, never a shared implementation. `fn spawnable<S: SendEventStore +
Sync>(s: &S)` asserting `Send` on `s.head()` compiles; the identical body bound
on bare `EventStore` fails with `error[E0277]`. The split buys exactly what it
claims.

**The bare flavour now has an implementer.** `LocalMemoryEventStore` —
`Rc<RefCell<Vec<SequencedEvent>>>` — implements `EventStore` directly in
`happenstance-testkit`, a genuinely downstream crate, alongside the blanket
`impl<T: SendEventStore> EventStore for T`. No `error[E0119]`. It passes all
twenty-seven rules under three harnesses. That is ES-7 corroborated and ADR-0001's
own lift condition met.

Two things about that store are worth recording because both were assumed
otherwise. `RefCell` alone is **`Send`** — it surrenders `Sync`, not `Send` — so a
store that is literally `RefCell<Vec<_>>` would prove nothing; `Rc` is what does
the work, and CF-28's wording should say so. And `#[tokio::test]` drives a
`!Send` store perfectly well: `tokio::spawn` requires `Send`, `Runtime::block_on`
does not, and the attribute expands to the latter. CF-23's case for making the
test wrapper a parameter stands, but its reason is wasm portability, not
`Send`-ness.

**The associated type cannot be varied between flavours.** ES-5 asserts there is
no mechanism; five spellings were tried and all five fail. Naming the associated
type in the attribute dies at `expected '+'` — the macro grammar is literally
`Ident : TraitBound (+ TraitBound)*`. A supertrait owning a stricter `Error`
yields two `Error`s and `E0221`. A where-clause on the bare trait is the
informative one: `mk_variant` builds the variant with `..tr.clone()`, so the
where-clause is copied onto *both* flavours, and it additionally breaks the
generated blanket impl with `error[E0275]: overflow evaluating the requirement`.
ES-5's conclusion holds; its cited mechanism is incomplete, since it credits
`transform_item` and the copying is `mk_variant`'s.

### Where the two ports diverge, and why one scheme still fits

The scheme is shared. The **provided-method budget is not**, and that asymmetry
is this ADR's most useful finding because it is invisible from `EventStore`.

`ProjectionStore` has a GAT, `type Batch<'a> where Self: 'a`. A provided body
that merely holds `&self` across an await behaves exactly as on `EventStore`. A
provided body that also holds the *batch* across a suspension point does not
compile at all, and there is no remedy: the contract crate reports *"future
cannot be sent between threads safely … `Send` is not implemented for
`<Self as SendProjectionStore>::Batch<'_>`"*, rustc suggests a bound that cannot
be written in a cloned body, and `for<'a> Self::Batch<'a>: Send` fails with
`E0311` on `TraitVariantBlanketType`. Only `Self: Sync + 'static, for<'a>
Self::Batch<'a>: Send` compiles — and that clause is cloned onto the `!Send`
flavour, where it then rejects an `Rc` batch.

The cause was isolated with three local traits under the same attribute: no GAT
works, a GAT *without* `where Self: 'a` works, a GAT *with* `where Self: 'a`
gives `E0311`. `EventStore` structurally cannot exhibit this. So the answer to
PS-35's question — must the two ports get one scheme? — is yes, and the reason a
single document is required is that the *consequences* differ and only a reader
holding both halves can see it.

A second, blunter cost lands on both ports. **`Self: Sync` makes a provided
method uncallable on a `RefCell`-backed store**, which is the shape the bare
flavour exists to serve: `E0277` at the call. Provided methods are therefore a
native-leaning convenience on ports whose reason for existing is the
single-threaded target. There is one escape and it is worth knowing: a method
that takes its arguments as *parameters* can build the stream before the async
block, so the future captures owned values rather than `&self` and needs no
`Sync` at all. The escape closes when the method constructs its own `Query`,
because edition-2024 RPITIT ties `read`'s stream to the query's lifetime
(`E0716`, then `E0597`). Phase 4 designs `head`/`count` and should spend that
lever deliberately.

## Consequences

**Good.** One definition per port, both targets, and the second port is now
decided rather than copied. `SendProjectionStore` is covered by an ADR for the
first time since it was written.

**Good.** ADR-0001 loses `provisional`. Its lift condition — a `!Send` reference
store passing the suite — is met in-tree. The *full* proof remains the Cloudflare
adapter at [phase 9](../RUNBOOK.md#phase-9--cloudflare-durable-object), and
nothing here anticipates it: what is proved is that the design admits a `!Send`
implementer and that the suite can drive one, not that a real platform SDK fits.

**Bad.** Provided methods are second-class on the bare flavour. Any port method
that could plausibly be called from a `!Send` adapter should be *required*, or
should take its arguments as parameters. This is a real constraint on phase 4 and
is not visible from the attribute.

**Bad, and load-bearing for phase 6.** A `ProjectionStore` provided method may
not hold its batch across a suspension point, and the diagnostic for violating it
arrives at the port with a suggestion that cannot be applied. Phase 6 freezes
this port; it should decide whether the GAT survives, knowing that the
`Self::Batch<'_>` spelling trap outlives the change everyone expects to retire it
— a fully **owned** batch (`type Batch<'a> = Owned;`) still gets `error[E0195]`
unless the impl writes `Self::Batch<'_>`, because the trap is about the count of
elided lifetimes in the signature and not about what the batch contains.

**Neutral.** `Error`'s bound (ES-6) stays deferred, and this ADR deliberately
does not settle it. It records two things for the phase that does. The current
declaration genuinely admits a `!Send` error on the *derived* flavour: the
failure is not at the declaration and not in the future, but at a call site
demanding `F::Output: Send`, tracing `Result<_, AppendError<E>>`. So
`store_error_crosses_a_join_handle`, when written, must assert on the output type
— a rule that only checks the future is `Send` passes against a `!Send` error and
is decorative. And there is a third option neither prior document weighed: a
`ThreadSafeEventStore: SendEventStore<Error: Send + Sync>` marker with a blanket
impl compiles, keeps the derivation and the blanket impl, and lets generic code
demand the stronger property without the wasm target paying for a native concern.

## Alternatives rejected

- **Two hand-written traits per port, the `umadb-dcb` shape.** It is the only
  arrangement in which the two flavours' associated types could differ, which is
  the one thing the derivation forbids. Rejected on ADR-0001's original grounds —
  it doubles the trait surface and every adapter's maintenance burden — and on a
  new one: it loses the blanket impl, and the blanket impl is what makes "bind
  the weaker flavour" work at every call site.

- **A different scheme per port**, deriving `EventStore` and hand-writing
  `ProjectionStore` so its GAT could carry `Batch: Send` on one flavour only.
  Rejected: `trait_variant` copies associated-type bounds verbatim, so
  `type Batch: Send` would impose `Send` on the `!Send` flavour and break the
  wasm target — but hand-writing to escape that buys a port that two ADRs would
  then govern, at the exact seam where an application holds both stores at once.
  The cost is documentation (PS-36) rather than a second scheme.

- **`Sync` in the attribute** rather than at the point of use. One line, and it
  looks free. It is not: the whole bound list reaches `read`'s stream, and an
  adapter whose stream hides a `Cell` is rejected with `error[E0277]`.

  ES-3 offers a second argument for this rejection that does **not** survive
  compilation, and it is recorded here so nobody quotes it: the clause says
  `memory.rs:154` and the SQLite adapter "both write `+ Send` and would both need
  `+ Send + Sync`". They would not. Flipping the attribute and running
  `cargo check --workspace --all-features` produced *zero* errors and neither
  impl was touched — an RPITIT impl is not required to restate the trait's
  auto-trait bounds, and rustc checks the hidden type instead. The real hazard is
  sharper than the one written down: because the impl signature need not restate
  the bound, an adapter author who writes `+ Send` is silently held to
  `+ Send + Sync`, and learns of it only when some future hidden type fails, far
  from the cause.

## Amendments this decision owes the specification

Recorded here rather than applied silently, because three of the four touch
`[FROZEN]` clauses and this repository's rule is that a frozen clause changes by
ADR and not by edit. All four are corrections to *citations and rule adequacy*;
none changes a normative MUST.

- **ES-2's named rule is necessary but not sufficient.**
  `send_flavour_stream_is_send_in_generic_code` does not reject the refactor ES-2
  says it rejects. Under `async fn read(..) -> Result<impl Stream, E>` the
  outermost item is the *future*, `trait_variant` marks the future `Send`, and
  asserting `Send` on the call's result is discharged against the future. The
  clause's own account of the mechanism implies this; the rule was written as
  though it did not. `spawns_from_generic`, which holds the stream across an
  await, is what bites — with `the trait Send is not implemented for impl
  Stream<…>`. Both rules are now in `memory.rs` and ES-2 should name both.
- **ES-3's impl-site argument is false**, as recorded above. Its conclusion
  stands on the `Cell`-in-a-stream leg alone.
- **ES-5's cited mechanism is incomplete.** Add `mk_variant`'s `..tr.clone()`
  beside `transform_item`, since the where-clause is the first spelling anyone
  reaches for and it is the one `transform_item` does not explain.
- **PS-36's named rule cannot be pinned as specified.** It asks for a
  `compile_fail` doctest "showing the diagnostic". The diagnostic carries no
  error code at all — `--message-format=json` reports `code: None` — and rustdoc
  on stable 1.97.1 *silently ignores* a `compile_fail,E0308` annotation, passing
  a doctest annotated with a code it demonstrably does not have. The doctest is
  worth keeping as documentation; it is not a gate. Pinning it needs a
  `trybuild`-style stderr snapshot, which is a dependency decision phase 6 owns.
