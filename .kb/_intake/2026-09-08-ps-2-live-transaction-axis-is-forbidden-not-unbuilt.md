# PS-2's live-transaction axis end is forbidden by the port, not merely unbuilt

**Date:** 2026-09-08
**Kind:** finding against a `[FROZEN]` clause — reported, not settled
**Found by:** writing `PostgresProjectionStore`'s bodies at phase 10b
**Owner:** PS-2's, not this adapter's

## The claim being corrected

PS-2 is `[FROZEN]`. Its Rule names *"one adapter holding a live transaction
(rusqlite or `sqlx`)"* as the far end of the batch-shape axis **still to be
built**, and the whole instrument-portfolio argument (CF-25) treats that end as
absent because nobody has got to it.

**It is not absent for want of effort. For the two drivers the clause names, the
port's own signatures forbid it**, and each is forbidden by a different
mechanism — which is why neither refutation generalises from the other, and why
finding one did not predict the other.

## `sqlx`

`ProjectionStore::begin` is **total, synchronous and infallible**:

```rust
fn begin(&self) -> Self::Batch;
```

`sqlx`'s only constructor for a transaction is `Transaction::begin`, which is
`async` and fallible; `Transaction`'s fields are private, so the type cannot be
assembled by hand. `Pool::try_acquire` is synchronous but yields a
`PoolConnection`, and `BEGIN` is still a round trip. **There is no total
synchronous expression of type `Transaction<'static, Postgres>`.**

`ProjectionProbe::probe_write` and `probe_delete_all` close the second door
independently: both are synchronous and infallible, so even handed a live
transaction there is no point at which an async driver's adapter could issue a
statement into it.

## `rusqlite`

`Connection` is `!Sync` and `Transaction<'_>` is `!Send`, so binding a live
handle costs the `SendProjectionStore` impl — the adapter drops to the bare
flavour to hold a transaction. `happenstance-sqlite` records this in its own
module documentation and chose the buffered batch for it.

## Why no compiler said so for a whole phase

`happenstance-postgres` declared `type Batch = sqlx::Transaction<'static,
Postgres>` with five `todo!()` bodies, and it **type-checked**, because `todo!()`
has type `!` and `!` coerces to everything. The skeleton was cited — in the
repository map, in the constitution's RS-90-1, and as phase 6's freeze evidence —
as an example of a skeleton declaring its driver's *real* types rather than
placeholders. It did. The type was real and uninhabitable, and a skeleton cannot
tell those apart.

The crate's own comment made the contradiction explicit without noticing it:
*"this adapter's real body will acquire the pooled connection at the first
statement rather than here."* With `Batch = Transaction` there is no
first-statement seam inside the adapter, because `begin` must **return** the
transaction.

## What this does and does not settle

It **does not** say PS-2 is wrong. The clause's substance — that a port frozen
against buffering adapters alone is shaped like a buffering adapter — is
untouched, and the exposure it records is real.

It **does** say the exposure cannot be discharged the way the clause anticipates.
Someone owns a choice between at least these, and it is not this adapter's:

1. **Accept it and reword.** State that the axis end is unreachable for
   `rusqlite` and `sqlx`, and name what a live-transaction adapter would have to
   be instead — a synchronous driver whose transaction type is `Send`, which is a
   narrower population than "a real database".
2. **Move the port.** `async fn begin() -> Result<Self::Batch, Self::Error>`
   admits the axis end. It is a breaking change to a `[FROZEN]` surface, and
   PS-6's argument against it — *"`async fn begin()` implies a round trip"* — is
   the reason it is shaped as it is; that argument is still good, and this is the
   cost of it stated for the first time.
3. **Leave it and record the falsifier as unfalsifiable in its stated terms**,
   which is honest but is the option that most needs writing down, because a
   provisional group waiting on an adapter nobody can build waits forever.

`experiments/live-handle-projection-batch/` already holds a compiled
counter-example on the borrowed-handle question and is the natural place for a
measurement if option 2 is taken seriously.

## Consequences already visible

- `standards/rust/90-skeletons-and-todo.md`'s RS-90-1 used this exact binding as
  its exemplar of a good skeleton. The atom now records that the exemplar was
  refuted, because *a real associated type is necessary and still not sufficient*
  is the sharper lesson and is what makes RS-90-1 and RS-90-2 two rules rather
  than one stated twice.
- Thirteen `[PROVISIONAL]` clauses are gated on PS-2 alone. None of them moves on
  this finding — the finding is about *how* the gate can be opened, not about
  opening it.
