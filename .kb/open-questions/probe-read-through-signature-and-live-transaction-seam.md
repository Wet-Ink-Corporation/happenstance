---
id: kb-open-question-probe-read-through-signature-001
title: probe_read_through's signature cannot be implemented correctly by a live transaction
kind: open_question
status: accepted
authority_tier: note
summary: >-
  crates/happenstance-core/src/projection.rs declares fn
  probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>
  — synchronous, infallible, shared borrow. A compiled test
  (probe_live_transaction_shape.rs) builds the live-transaction end of the
  batch-shape axis PS-2 [FROZEN] requires and runs every body the current
  signature admits: taking &Self::Batch and issuing a statement is E0596
  (pinned as compile_fail), conceding &mut and awaiting is E0728 (pinned
  the same way), and the only bodies that compile either declare
  READS_THROUGH_BATCH = false while the batch actually reads its own
  uncommitted writes (a false statement a test catches), or answer from
  committed state only, which is the exact reading PS-12 forbids. Every
  READS_THROUGH_BATCH = true implementor in the workspace today answers
  from an in-process map or buffer; the one adapter at the live-transaction
  end (SQLite's projection store) declares false with an unimplemented!()
  body. ADR-0036 recorded PS-2's part 2 (a live-transaction adapter) as
  unmet and attributed the gap to scarcity — nothing built yet; this atom
  adds a second, compiled cause the ADR could not have had: the port's own
  probe cannot be implemented by that end under today's signature, so
  building toward it has a standing incentive to buffer instead, which is
  the monoculture PS-2 exists to prevent. Recommended fix (not yet
  decided, and not this lane's to make): move probe_read_through to
  &mut Self::Batch, hand-desugared impl Future<Output =
  Result<Option<u64>, Self::Error>>, no Send bound — changing five in-tree
  implementors across three crates and one example, all behind the
  conformance/unstable-projection semver exemption ADR-0036 already grants.
  The corollary question is whether PS-2's live-transaction bar can be
  judged met by an adapter that declares the capability false to compile
  against the current shape. Phase 10b and ADR-0060 replace this atom's
  attributed cause without touching its gap. The absence at PS-2's
  live-transaction end is not scarcity and is no longer only a probe
  defect: ProjectionStore::begin is total, synchronous and infallible,
  sqlx's only constructor is async and fallible with private fields so no
  total synchronous expression of Transaction<'static, Postgres> exists,
  and rusqlite's Transaction<'_> is !Send and costs the SendProjectionStore
  impl — two independent mechanisms, which is why finding one did not
  predict the other. The honest scope is therefore the whole probe seam
  rather than one method: probe_write and probe_delete_all are synchronous
  and infallible too, so even handed a live transaction there is no seam
  for an async driver's adapter to issue a statement into, and the
  recommended &mut/async/Result move on probe_read_through is not
  sufficient alone. And the consequence for PS-2's bar is sharper than a
  coverage hole: a conformant live-transaction adapter must declare
  READS_THROUGH_BATCH = false and therefore reports the same capability
  profile as a buffering one, so the suite cannot distinguish the two ends
  of the axis the clause asks for adapters at. ADR-0060 (kb-decision-0060)
  keeps the port's gate on exactly this ground and declines the signature
  change, stating it is PS-2's owner's call rather than an adapter lane's —
  so this question stays open, with a wider scope than when it was written.
depends_on: []
related:
  - kb-decision-0036
  - kb-decision-0060
  - kb-decision-0017
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-decision-0025
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/probe-read-through-and-the-live-transaction-end.md
  - .kb/_intake/2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md
last_reviewed: 2026-09-09
---

# probe_read_through's signature cannot be implemented correctly by a live transaction

## What is true today

`spec/SPECIFICATION.md`'s **PS-2** (`[FROZEN]`) gates the projection port's
freeze on the conformance suite going green against adapters at both ends
of the batch-shape axis: "the suite green against one adapter holding a
live transaction (rusqlite or `sqlx`) and one that cannot hold anything
across an await." ADR-0036 (accepted, `.kb/decisions/0036-the-projection-
port-ships-gated.md`) evaluated this bar and recorded part 2 as unmet,
attributing the gap to `SqliteProjectionStore`'s `Batch` being an *owned*
write set under `kb-decision-0017` — statements pushed and replayed at
commit, sitting at the buffered end rather than the live-transaction end.
That reads the absence as scarcity: nobody has built the adapter yet.

**Scarcity is not even a contributing cause.** Writing
`PostgresProjectionStore`'s bodies at phase 10b produced a compiled
refutation for each of the two drivers PS-2 names, by *independent*
mechanisms — which is why finding one did not predict the other.
`ProjectionStore::begin` is total, synchronous and infallible; `sqlx`'s
only transaction constructor is `async` and fallible and `Transaction`'s
fields are private, so no total synchronous expression of
`Transaction<'static, Postgres>` exists, and `Pool::try_acquire` leaves
`BEGIN` a round trip regardless. `rusqlite`'s `Transaction<'_>` is `!Send`,
which costs the `SendProjectionStore` impl outright — the reason
`happenstance-sqlite` chose a buffered batch, and it says so in its own
module documentation. No compiler said any of this for a whole phase,
because `happenstance-postgres` declared `type Batch =
sqlx::Transaction<'static, Postgres>` with five `todo!()` bodies and it
type-checked: `todo!()` has type `!`, `!` coerces to everything, and a
real-but-uninhabitable type is indistinguishable from a real one to a
skeleton.

`crates/happenstance-core/src/projection.rs` declares:

```rust
fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
```

Not `async`, not fallible, a shared borrow. The sibling member on the same
trait already carries the reasoning for why a store-touching probe must be
a future — real I/O against the store, unlike the other three members
which mutate a batch the caller already holds — and `probe_read_through`
is exactly that whenever the batch is a live transaction.

A compiled test, `crates/happenstance-core/tests/
probe_live_transaction_shape.rs`, builds a `LiveTransactionStore` whose
batch issues statements as they are made (`sqlx`'s `Executor for &mut
Transaction` in miniature) and runs every body the declared signature
admits:

| Body | Outcome |
| --- | --- |
| Take `&Self::Batch`, issue a statement | `E0596`, pinned as `compile_fail` |
| Concede `&mut`, `.await` the statement | `E0728`, pinned the same way |
| Declare `READS_THROUGH_BATCH = false` | Compiles — a false statement about the store, caught by a test |
| Answer from committed state only | Compiles — returns `None` for a row PS-12 requires visible |
| Block on the future | Panics: cannot start a runtime from within a runtime |

Every `READS_THROUGH_BATCH = true` implementor in the workspace today
answers from an in-process map or buffer. The one adapter at the
live-transaction end — `happenstance-sqlite`'s projection store — declares
`false` with an `unimplemented!()` body. So PS-12's
`batch_reads_reflect_pending_writes` rule has only ever executed against
in-process stores, and the reported-skip mechanism that would normally
flag this hides it, because the fixture's stated reason is accepted at
face value.

## The scope is the probe seam, not one method

`probe_write` and `probe_delete_all` are synchronous and infallible too.
So even a store *handed* a live transaction has no point at which an async
driver's adapter could issue a statement into it, and the `&mut` / `async`
/ `Result` move on `probe_read_through` alone does not open the axis end.
An atom recommending a one-method fix that does not work would be worse
than one recommending nothing, so the recommendation below is restated at
seam grain.

The consequence for PS-2's own bar is sharper than a coverage hole. A
store whose batch genuinely *is* a live transaction can implement the port
today — it must simply declare `READS_THROUGH_BATCH = false`. It then
reports **the same capability profile as a buffering adapter**, so the
conformance suite cannot tell the two ends of the axis apart. The clause
asks for adapters at opposite ends of something the instrument does not
measure.

## The question

Does the probe seam move — `probe_read_through` to `&mut Self::Batch`,
hand-desugared `impl Future<Output = Result<Option<u64>, Self::Error>>`,
no `Send` bound, and `probe_write` / `probe_delete_all` with it — or does
PS-2 get reworded to say the axis end is unreachable for `rusqlite` and
`sqlx` and to name what a live-transaction adapter would have to be
instead (a synchronous driver whose transaction type is `Send`, a much
narrower population than "a real database"), or does the falsifier get
recorded as unfalsifiable in its stated terms?

A fourth option is narrower: split `READS_THROUGH_BATCH` into "can" and
"can be asked synchronously," which removes the lie without removing the
coverage hole — and, on the finding above, without making the ends
distinguishable either.

**ADR-0060 answered the part that was its to answer and left this open.**
It keeps the port's `unstable-projection` gate on exactly this ground —
freezing `begin`, `probe_write` and `probe_read_through` as they stand
would make a semver promise out of the signatures that forbid the second
shape — and it *declines* the signature change, because it is breaking to
a trait testkit consumers implement and because it is PS-2's owner's call
rather than an adapter lane's to make in passing.

## Why this is not a routine signature tweak

The change is breaking to a public trait of a published crate
(`happenstance-core`) and touches five implementors across three crates
and one example, but the trait sits behind
`conformance`/`unstable-projection`, which ADR-0036 documents as carrying
no semver promise, so the break is exempt and cheap regardless of timing.
The counter-argument originally recorded against moving early — write the
real adapter first and let it tell you what the seam needs — has now been
run: four adapters have passed the projection suite (a file, a pooled
server, a one-shot HTTP proxy, an embedded graph database), and what they
told the seam is the finding above.
`experiments/live-handle-projection-batch/` holds the compiled
counter-example on the borrowed-handle question and is where a measurement
for a moved seam belongs.

## Cost of delay

Lower than it was, and differently shaped. The original deadline was
whichever came first of the Postgres projection store being written (phase
10) or the port being frozen (phase 6); phase 10b has happened and the
port did not freeze. ADR-0060's gate does not expire, so nothing forces a
date — the standing cost is that every adapter built meanwhile has an
incentive to buffer, because buffering is the only way to declare the
capability truthfully, which is the monoculture PS-2 exists to prevent.

## What this does not settle

Whether PS-2's part 2 is met by any particular adapter — that verdict
belongs to the clause's owner, and ADR-0060 explicitly leaves it there.
Whether `spec/SPECIFICATION.md` needs an amendment beyond ADR-0060's
amendment to PS-2's `Rule` (the MUST, the maturity marker and the `Cases`
are untouched). `READS_THROUGH_BATCH`'s own shape, and what a
live-transaction adapter's `Batch` should be if one is ever admissible.
