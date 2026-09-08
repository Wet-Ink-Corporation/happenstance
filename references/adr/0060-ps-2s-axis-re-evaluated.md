# ADR-0060 — PS-2's axis re-evaluated: one end occupied, the other unobservable

- **Status:** proposed
- **Date:** 2026-09-08
- **Phase:** 12 (the `0.2.0` release pass)
- **Re-evaluates:** [ADR-0036](../../.kb/decisions/0036-the-projection-port-ships-gated.md), whose decision it **reaffirms on a different ground**
- **Amends:** PS-2's `Rule` in `spec/SPECIFICATION.md` §4
- **Evidence:** `crates/happenstance-core/tests/probe_live_transaction_shape.rs`; phases 10b and 11

## The question

PS-2 is `[FROZEN]` and says the port MUST NOT be frozen until the suite has failed
a hostile store **and** *"two adapters at opposite ends of the batch-shape axis
have passed it"*. Its `Rule` names those ends: *"one adapter holding a live
transaction (rusqlite or `sqlx`) and one that cannot hold anything across an await
(Workers `SqlStorage` or Neon over one-shot HTTP)."*

ADR-0036 applied that bar at phase 6 and found part 2 unmet, with the words *"PS-2
is applied, not amended."* Phases 10b and 11 have since changed the facts under it
twice, in opposite directions. **What does the bar say now, and does the port still
ship gated?**

## What changed since ADR-0036

**The "cannot hold anything across an await" end is occupied.** ADR-0036 recorded
that *"no cannot-hold-across-await adapter has passed at all."* `happenstance-neon`
is now exactly that adapter — no connection, no interactive transaction, no cursor,
one round trip per operation — and it passes the projection suite against a live
endpoint. This is the first time either end of PS-2's axis has been occupied by
anything, and it is the half of part 2 the clause named a specific technology for.

**The adapter population went from one to four.** `SqliteProjectionStore`,
`PostgresProjectionStore`, `NeonProjectionStore` and `LadybugProjectionStore` all
run `projection_store_conformance!` against real backing stores — a file, a pooled
server, a one-shot HTTP proxy, and an embedded graph database addressed in Cypher.
Phase 11's verdict was pre-registered and its third condition — *"`GraphWriteSet`
needs a field the port cannot express, or `commit` an argument it is not given"* —
**did not fire**. Four independent implementations found nothing missing.

**And the live-transaction end got worse than unbuilt**, in two steps.

## Step one: the named drivers are refuted, each differently

Phase 10b tried to write `PostgresProjectionStore` against the binding the
skeleton declared, `type Batch = sqlx::Transaction<'static, Postgres>`, and could
not:

- **`sqlx`.** `begin` is total, synchronous and infallible (PS-6). `sqlx`'s only
  route to a `Transaction` is `async` and fallible, and its fields are private so
  the value cannot be assembled by hand. `Pool::try_acquire` is synchronous and
  yields a *connection*; `BEGIN` is still a round trip. There is no total
  synchronous expression of the type.
- **`rusqlite`.** `Transaction<'_>` is `!Send`, so binding it costs the
  `SendProjectionStore` impl — which is why `happenstance-sqlite` buffers.

Five `todo!()` bodies type-checked against the impossible binding for a whole
phase, because `todo!()` has type `!` and `!` coerces to everything. That is
recorded separately; what matters here is that **both drivers PS-2 names are
refuted, by different mechanisms**, so neither refutation predicted the other.

## Step two, and it is the finding: the two ends are indistinguishable to the suite

A live-transaction adapter is not merely hard to build. **If built, the
conformance suite cannot tell it apart from a buffering one.**

`crates/happenstance-core/tests/probe_live_transaction_shape.rs` establishes this
by construction, and predates this ADR. `LiveTransactionStore`'s batch *is* a
transaction: statements are issued as they are made, against a connection the
batch owns. It implements the port. And it must declare

```rust
const READS_THROUGH_BATCH: bool = false;
```

which that file's own comment calls **"a false statement about this store"** — its
transaction reads its own uncommitted writes by construction, and a test in the
same file runs that property and shows it holds. The reason it must lie is one
signature:

```rust
fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
```

Synchronous, infallible, and `&Self::Batch`. A driver borrows its connection
**mutably** to issue a statement, and the statement is I/O. So the store's
implementation is `unimplemented!("a live transaction cannot be read
synchronously or infallibly")`.

`ProjectionProbe::probe_write` closes the same door from the other side: it too is
synchronous and infallible, so a live-transaction store satisfies it only by
writing into a buffer the batch owns — which is the buffering shape wearing the
other end's name.

**Therefore:** the one capability that distinguishes the two ends of the
batch-shape axis is unreachable, and a conformant live-transaction adapter reports
the same capability profile as `SqliteProjectionStore`. PS-2 asks for the suite
green at two opposite ends of an axis **the suite has no way to observe**.

The bar is not merely unmet. As written it is **unobservable**, and would remain
unobservable on the day somebody built the adapter.

## Decision

### §1 PS-2's `Rule` is amended to say what the ends are and are not

The clause's MUST is untouched — the port is still not frozen until two ends pass.
What changes is the `Rule`, which currently names two technologies as though
picking them were the remaining work. It now records that one end is occupied,
that the other is unreachable for both drivers it named, and what an adapter would
have to be instead. **No maturity marker moves**, so §7.2's census is unchanged.

### §2 The port keeps its gate at `0.2.0`, on a stronger ground than ADR-0036's

ADR-0036 gated the port because *only one adapter had run the suite, at one end*.
**That reason has expired**: four adapters run it, and one of PS-2's two named ends
is occupied. Re-evaluated honestly, the evidence for the port is materially better
than it was at phase 6.

**The gate stays anyway, and the reason is the finding above rather than the
count.** Lifting `unstable-projection` makes `ProjectionStore` a semver promise.
The signatures that promise would freeze are `begin` (total, synchronous,
infallible), `probe_write` (synchronous, infallible) and `probe_read_through`
(synchronous, infallible, `&Batch`) — and those are **precisely the signatures this
ADR has just shown to be what forbids the second batch shape from existing or from
reporting itself**.

Freezing them now would make a promise out of the defect. That is a sharper reason
to wait than "we have not seen enough adapters", and it is the one that would
survive a fifth adapter arriving next week: more implementations at the same end do
not answer it, because the finding is not about scarcity.

The asymmetry ADR-0036 recorded is unchanged and still decides the tie: a frozen
port is a promise, a published version can be yanked but never removed, and the
cost of the arm taken is that a consumer types a feature name.

**PS-3 keeps `[PROVISIONAL]`.** A SHOULD that is still being satisfied is not a
moved marker — ADR-0036's own words, and they still apply.

### §3 `probe_read_through`'s signature is proposed for change, and this ADR does not make it

The mechanism is named because naming it is the point of the finding. The shape
that would let a live-transaction store answer truthfully is:

```rust
fn probe_read_through(
    &self,
    batch: &mut Self::Batch,
    key: &str,
) -> impl Future<Output = Result<Option<u64>, Self::Error>>;
```

`&mut` because a driver borrows its connection mutably to run a statement;
`async` because the statement yields; `Result` because it is I/O and can fail.
Spelled `-> impl Future` rather than `async fn` for the reason the trait's other
async member already is: `async fn` in a public trait fires `async_fn_in_trait`
under the gate's `-D warnings`, and the desugaring puts the *absence* of a `Send`
bound where a reader can see it.

**Three things this proposal is not.** It is not made here: `ProjectionProbe` is
`ProjectionStore`'s instrument, and moving it is PS-2's owner's call, which is
what `probe_live_transaction_shape.rs`'s own header already says. It is not free:
it is a breaking change to a trait `happenstance-testkit` consumers implement, and
every existing adapter's `unimplemented!()` body would have to change shape even
though none of them would start answering. And it is **not sufficient** on its own
— `probe_write` being synchronous and infallible still forces a live-transaction
store to buffer its writes, so the honest scope of the change is the whole probe
seam rather than one method.

What it *would* buy is the thing PS-2 needs and does not have: a live-transaction
adapter able to declare `READS_THROUGH_BATCH = true` truthfully, which is what
makes the two ends of the axis distinguishable and the bar observable.

## Consequences

- PS-2's `Rule` changes; its MUST, its maturity and its `Cases` do not.
- Three passages of supporting prose in `spec/SPECIFICATION.md` describing the
  port as having "zero adapters" or the live-transaction end as merely "unbuilt"
  are reconciled in the same change.
- ADR-0036's decision stands. Its stated *reason* is superseded by this one, and
  the two are recorded separately rather than merged, because "only one adapter has
  run the suite" was true when written and is the kind of sentence that would
  otherwise be quietly rewritten into something it never said.
- Nothing about the `0.2.0` release moves.

## Falsifier

This ADR is reopened by either of:

1. **A live-transaction adapter that passes the suite and can declare
   `READS_THROUGH_BATCH = true` truthfully.** That would mean the probe seam
   admits the shape after all and §2's reason to keep the gate has gone.
2. **A fifth adapter finding something the port cannot express.** §2 argues that
   more implementations at the buffered end answer nothing; a counter-example
   would show the population still has something to teach, which changes what the
   gate is for rather than whether it stays.

What does **not** reopen it: another adapter at the buffered end passing. That is
the case §2 is explicitly written to be indifferent to.
