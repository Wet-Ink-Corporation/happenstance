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
  Result<Option<u64>, Self::Error>>, no Send bound, before phase 10 —
  changing five in-tree implementors across three crates and one example,
  all behind the conformance/unstable-projection semver exemption ADR-0036
  already grants. The corollary question is whether PS-2's live-transaction
  bar can be judged met by an adapter that declares the capability false to
  compile against the current shape.
depends_on: []
related:
  - kb-decision-0036
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-ps-19-scope-narrower-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/probe-read-through-and-the-live-transaction-end.md
last_reviewed: 2026-09-07
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

There is already drift toward the wrong resolution: the Postgres
projection store's own story spec proposes a `Vec`-of-statements batch and
lists "makes `READS_THROUGH_BATCH = true` reachable by consulting the
buffer before the table" among its reasons — putting the second adapter at
the buffered end beside SQLite, the exact monoculture PS-2's `Rejects:`
paragraph names.

## The question

Does `probe_read_through`'s signature move to `&mut Self::Batch`,
hand-desugared `impl Future<Output = Result<Option<u64>, Self::Error>>`,
no `Send` bound — before the live-transaction adapter is written — or does
that adapter get built against today's shape and PS-2's part 2 get judged
on a capability the adapter had to declare false to compile?

A third option is narrower: split `READS_THROUGH_BATCH` into "can" and
"can be asked synchronously," which removes the lie without removing the
coverage hole.

## Why this is not a routine signature tweak

The change is breaking to a public trait of a published crate
(`happenstance-core`), touches five implementors across three crates and
one example, and PS-2 is `[FROZEN]` — but the trait sits behind
`conformance`/`unstable-projection`, which ADR-0036 already documents as
carrying no semver promise, so the break is exempt and cheap regardless of
timing. The counter-argument recorded against moving now: the port is
`[PROVISIONAL]` and the exemption does not expire, so amending a signature
against a hypothetical adapter risks the same freezing-against-instruments
error PS-2's own `Rejects:` paragraph forbids, one level up — write the
real adapter first and let it tell you what the seam needs. The answer
offered to that: the falsifying properties (`&` vs `&mut`, sync vs
`async`) are properties of every driver in the candidate set and are
compiler-checked here rather than reasoned about, and the adapter written
against today's shape has a standing incentive to buffer rather than hold
a live transaction, because buffering is the only way to declare the
capability truthfully.

## Cost of delay

Bounded by whichever comes first of the Postgres projection store being
written (phase 10) or the port being frozen (phase 6) — and those are in
the wrong order for leaving the signature as-is, because the freeze bar is
evaluated against the adapter set that exists at freeze time.

## What this does not settle

Whether PS-2's part 2 is met by any particular adapter — that verdict
belongs to the clause's owner. Whether `spec/SPECIFICATION.md` needs an
amendment (it does not, on this reading; PS-2's text is not in question,
only whether an adapter can satisfy the instrument that checks it).
`READS_THROUGH_BATCH`'s own shape, and what the Postgres adapter's `Batch`
should be.
