# `probe_read_through` is synchronous, infallible and takes `&Self::Batch`. Does the signature move before the live-transaction adapter is written, or does that adapter be written against it and PS-2's part 2 judged on a declined capability?

Short answer up front: **move the signature — `&mut Self::Batch`, hand-desugared
`impl Future`, `Result<Option<u64>, Self::Error>`, no `Send` bound — and do it
before phase 10, because the alternative is a `[FROZEN]` clause's bar being
recorded as met by an adapter whose defining property was never run.** But the
decision is not this lane's: `ProjectionProbe` is a public item of a published
crate, the change touches five implementors across three crates this lane may not
write, and PS-2 is `[FROZEN]`.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

`spec/SPECIFICATION.md` §4's **PS-2** is `[FROZEN]` and gates the port's freeze on
two adapters at opposite ends of the batch-shape axis. Its **Rule** names them
(`spec/SPECIFICATION.md`:4918-4920):

> the suite green against one adapter holding a live transaction (rusqlite or
> `sqlx`) and one that cannot hold anything across an await

ADR-0036, accepted 2026-09-02, evaluated that bar and found part 2 unmet
(`.kb/decisions/0036-the-projection-port-ships-gated.md`:77-88), attributing the
gap to what has been built:

> SQLite's `Batch` is an *owned* write set under `kb-decision-0017`, statements
> pushed and replayed at commit, so it sits at the buffered end rather than the
> live-transaction end
> (`.kb/decisions/0036-the-projection-port-ships-gated.md`:84-86)

That reads the absence as **scarcity** — a thing nobody has built yet. This brief
does not contradict it. It adds a second cause, which the ADR could not have had
because nothing in the tree recorded it: the port's own conformance seam cannot
be implemented by that end.

---

## What is true today, measured

`crates/happenstance-core/src/projection.rs` declares

```rust
fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
```

Not `async`, not fallible, `&Self::Batch` rather than `&mut`. Twenty-odd lines
above it the sibling member carries the crate's own reasoning for why a probe
that touches the store must be a future — *"the other three mutate a batch the
caller already holds, while this is real I/O against the store"* — and
`probe_read_through` is real I/O against the store whenever the batch is a live
transaction.

`crates/happenstance-core/tests/probe_live_transaction_shape.rs` (landed by this
lane) builds that end and runs every body the signature admits.
`LiveTransactionStore`'s batch issues its statements as they are made; reading one
back is `async fn select(&mut self, …) -> Result<Option<u64>, LiveError>`, which
is `sqlx`'s `Executor for &mut Transaction` in miniature.

| Body | Outcome, run |
| --- | --- |
| Take `&Self::Batch` and issue a statement | `error[E0596]`, pinned as a `compile_fail` doctest on the port |
| Concede `&mut`, `.await` the statement | `error[E0728]`, pinned the same way |
| Declare `READS_THROUGH_BATCH = false` | Compiles. **A false statement about the store**: the batch reads its own uncommitted writes, and a test asserts it does |
| Answer from committed state | Compiles. Returns `None` for a row the transaction can see — the reading PS-12 forbids |
| Block on the future | Panics: *"Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks."* The suite is always inside one |

Every `READS_THROUGH_BATCH = true` in the workspace is an in-process store
answering from a map or a buffer — `projection_memory.rs`, `fixtures.rs`, two
mutation-coverage stores, two in `examples/outside-projection-adapter`, one test
file. The one adapter that has implemented `ProjectionProbe` declares `false` and
leaves the body `unimplemented!()`. So **PS-12's rule
`batch_reads_reflect_pending_writes` has only ever executed against in-process
stores**, and the rule that would notice is emitted as a reported skip carrying
the fixture's own stated reason.

There is already drift toward the wrong resolution. The story spec for the
Postgres projection store proposes a `Vec`-of-statements batch and lists *"makes
`READS_THROUGH_BATCH = true` reachable by consulting the buffer before the
table"* among its reasons — which puts the second adapter at the buffered end
beside SQLite, which is the monoculture PS-2's `Rejects:` paragraph names.

---

## The question

**Does `probe_read_through`'s shape move before the live-transaction adapter is
built?**

### Option A — amend the signature now

```rust
fn probe_read_through(
    &self,
    batch: &mut Self::Batch,
    key: &str,
) -> impl Future<Output = Result<Option<u64>, Self::Error>>;
```

Hand-desugared rather than `async fn`, and with **no `+ Send`** — `probe_read`'s
own doc argues both at length and the argument reaches here unchanged: the trait
is not under `#[trait_variant::make]`, so `async fn` fires `async_fn_in_trait`
under `-D warnings`, and a `Send` bound would break `wasm32` and could not be
relaxed later.

- **Caller (the suite):** `batch_reads_reflect_pending_writes` gains an `.await`
  and a `?`. It already holds the batch mutably to call `probe_write`, so the
  `&mut` costs it nothing structurally.
- **Adapter author:** five in-tree implementors change, in three crates and one
  example. Four of them answer from a map and get `Ok(…)` and an `async` block
  for free; the fifth is SQLite's `unimplemented!()`, unchanged in substance.
  Out-of-tree there is one known implementor, in `examples/`.
- **Semver:** breaking to `ProjectionProbe`, and **exempt** — the trait is behind
  `conformance`, which implies `unstable-projection`, and the module documents the
  exemption. Genuinely cheap after `0.2.0` as well.
- **What it costs to be wrong:** an `&mut` where `&` would do, on a trait only a
  conformance suite calls. Small.

### Option B — leave the shape and judge PS-2 on what the adapter declares

The live-transaction adapter is written, declares `READS_THROUGH_BATCH = false`,
takes the read-through rule as a reported skip, goes green, and PS-2 part 2 is
recorded as met.

- **Caller:** nothing changes.
- **Adapter author:** must write a false statement about their own store to pass.
- **Semver:** none.
- **What it costs to be wrong:** the port freezes with the one rule that
  distinguishes the two ends of its own axis never having run at the
  live-transaction end.
  A frozen port is a semver promise and a published version can be yanked but not
  removed — ADR-0036's own asymmetry argument, applied to the thing ADR-0036 was
  deciding.

### Option C — amend the capability instead of the method

Split `READS_THROUGH_BATCH` into "can" and "can be asked synchronously", so the
declaration stops being false while the signature stands.

- Costs a second const on the fixture surface and does not let the rule run. It
  buys honesty in the CI output and nothing in coverage. Recorded because it is
  the cheapest thing that removes the *lie* without removing the *hole*, and
  someone will propose it.

---

## Recommendation, and the strongest argument against it

**Option A, sequenced before phase 10.**

The strongest argument against it, in its own words: *the port is
`[PROVISIONAL]` and the exemption does not expire, so nothing forces this now;
amending a signature against a hypothetical adapter is exactly the freezing-
against-instruments error PS-2's `Rejects` paragraph names, one level up. Write
the Postgres adapter first, let it tell you what the seam needs, and change the
seam against a real store rather than against `LiveTransactionStore`.*

That argument is good and it is why this is a recommendation rather than a
change. Two things answer it. First, the falsifier is not hypothetical about the
part that matters: `&` versus `&mut` and sync versus `async` are properties of
every driver in the candidate set, and both halves are compiler-checked rather
than reasoned. Second, the ordering is asymmetric — the adapter written against
today's shape has a standing incentive to buffer, because buffering is the only
way to declare the capability truthfully, and the drift toward that is already in
the backlog.

**Confidence: medium-high on the diagnosis, medium on the timing.** The diagnosis
is compiled. The timing rests on a judgement about which of two irreversibilities
is worse, and reasonable people weight those differently.

---

## Cost of delay

Not `0.2.0`. The exemption is real and the trait is gated. The deadline is
**whichever comes first of** the Postgres projection store being written (phase
10) and the port being frozen (phase 6) — and the two are in the wrong order for
Option B, because the freeze bar is evaluated against the adapter set that
exists.

---

## What this does not settle

- **Whether PS-2's part 2 is met by any particular adapter.** That is the
  clause's owner's, and this brief supplies evidence for it, not a verdict.
- **Whether `spec/SPECIFICATION.md` needs an amendment.** It does not, on this
  reading: PS-2 says what it says and this is about whether an adapter *can*
  satisfy the instrument that checks it. §6.5's portfolio table calls the
  live-transaction shape the axis's **near** end and the buffered shape its far
  end; this brief says "the live-transaction end" throughout, so the two
  vocabularies cannot be read as disagreeing. Nothing here proposes moving a maturity
  marker or a `[FROZEN]` clause's text.
- **`READS_THROUGH_BATCH`'s own shape.** Option C is recorded, not argued.
- **What the Postgres adapter's `Batch` should be.** That story is written and
  proposes a buffer; this brief says the reason it gives for that choice is a
  consequence of the seam, not an independent argument for the shape.

---

## Handoff

- **File:** `crates/happenstance-core/src/projection.rs`
- **Anchor:** `fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;`
- **Proposed text:**

  ```rust
  fn probe_read_through(
      &self,
      batch: &mut Self::Batch,
      key: &str,
  ) -> impl Future<Output = Result<Option<u64>, Self::Error>>;
  ```

- **Follows in the same change**, all outside this lane's writable surface:
  `crates/happenstance-testkit/src/fixtures.rs` and the projection rule
  `batch_reads_reflect_pending_writes` that calls it;
  `crates/happenstance-sqlite/src/projection_store.rs`'s `unimplemented!()` arm;
  `crates/happenstance-testkit/tests/projection_mutation_coverage/`'s two stores;
  `examples/outside-projection-adapter/src/lib.rs`.
- **What lands with it:** `crates/happenstance-core/tests/probe_live_transaction_shape.rs`
  flips from recording an obstacle to registering a passing store, and the
  section on `probe_read_through`'s page goes with the signature — a test pins
  the two together, so neither can move alone.

## Revision record

Revision 1, authored 2026-09-04 by the `lane/core-ports` remediation lane against
finding F1-02, in the same session as the evidence it cites. No independent
critic. Nothing was withdrawn, because nothing has been argued against it yet.
