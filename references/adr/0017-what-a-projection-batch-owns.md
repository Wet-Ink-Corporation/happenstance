# ADR-0017: What a projection batch owns, and the seam that is not a write vocabulary

- **Status:** accepted
- **Date:** 2026-08-13
- **Settles:** PS-4 – PS-15 (`spec/SPECIFICATION.md:4849-5171`)
- **Answers:** `kb-open-question-projection-batch-no-apply-001`
  (`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`),
  sub-questions 1, 2 and 4. Sub-question 3 — whether closing this retroactively
  validates ADR-0006's encoding-versus-orchestration discriminator — stays open
  with the typed layer (HS-P0011).
- **Builds on:** ADR-0003 (opaque payloads), ADR-0007 (the projection runner
  decodes), ADR-0008 (one derivation for both ports), ADR-0010 (the suite must
  prove itself). Contradicts none of them; corrects one sentence of ADR-0007's
  Context, per PS-32, and that correction is a superseding atom's rather than an
  edit's.
- **Evidence:** `references/adapter-shapes.md`,
  `crates/happenstance-ladybug/src/live_handle.rs`,
  `references/evaluation/ps-clause-pairing-sweep.md`

## The question, as the runbook posed it

> **0017** | 6 | What does a projection batch own, what vocabulary writes into
> it, and what happens when it is dropped? (PS-4 – PS-15)
> — `RUNBOOK.md:296`

Three halves. `kb-playbook-one-decision-per-adr-title-001` records that three
times in two days an "and" in an ADR title carried a strong decision and a weak
one, and the weak one was reversed within days — so each half is graded here
before it is decided, and the discriminator is not the conjunction but what each
half rests on.

| Half | Rests on | Strength |
|---|---|---|
| what a batch **owns** | two compiler transcripts and one ICE, all reproduced at 1.97.1 | **settled by being made** |
| what **vocabulary** writes into it | a consumer count — one generic consumer today, and a named falsifier if a second lands | **provisional, falsifier named, phase named** |
| what happens when it is **dropped** | a reviewer's probe that already found the failure, plus the absence of async `Drop` in Rust | **settled by being made** |

The middle half is the one this ADR marks provisional rather than splitting,
because it is not a claim about code that does not exist — it is a claim about
how many consumers do, and the observation that would refute it is stated in the
clause itself (PS-9's falsifier) and evaluated at a named phase.

## Context

### What the compiler said, quoted rather than paraphrased

Phase 2 built six adapter skeletons *because a port's shape is falsified by a
type, not by a behaviour* (`references/adapter-shapes.md:7-8`). Four of its
transcripts bear on this ADR, and the fourth is the one that decides it.

**1. rusqlite's live transaction is rejected twice over, and the split is the
finding** (`references/adapter-shapes.md:61-102`):

> **Six errors, and the split is the finding.**
>
> *Reason 1 — the store.* `rusqlite::Connection` is `Send` and **not** `Sync`, so
> `&Self` is not `Send`, so every future in the trait is rejected — **including
> `checkpoint`, which never touches a batch at all**.
>
> *Reason 2 — the batch.* … `commit` and `rollback` still do not [compile],
> because `rusqlite::Transaction<'_>` is itself `!Send` and they are rejected
> **on the parameter alone**.

and, at `:97-102`:

> **These six diagnostics carry no error code.** `--message-format=json` reports
> `code: None` on every one. That is not sloppiness in the transcript: there is
> no code to quote. … a `compile_fail,E….` doctest cannot pin any of them, so
> pinning needs a `trybuild`-style stderr snapshot, which is the dependency
> decision **phase 6 owns**.

**2. The one adapter that could most plausibly have demanded a borrow did not**
(`:160-167`):

> `happenstance-postgres` binds an owned, `Send`, `'static` transaction to
> today's GAT (`projection_store.rs:100`). The lifetime parameter is
> *satisfiable and unused*. This is the one adapter with a pooled network
> connection and a real interactive transaction, so it is the one that could most
> plausibly have demanded a borrow — and it did not, because `Pool::begin()`
> hands back a transaction that owns its `PoolConnection`.

**3. A borrowed GAT was accepted, and it refutes the `Send` argument**
(`:169-179`):

> `happenstance-ladybug` ships the owned `GraphWriteSet` as its working
> hypothesis *and* a second, compiling, GAT-**borrowed** impl
> (`live_handle.rs:174`), exercised across a real `tokio::spawn`. So the borrowed
> GAT does **not** fail on the `Send` flavour. … **Phase 6 must weigh it**: the
> rusqlite rejection above is about `rusqlite::Connection` being `!Sync`, not
> about GATs, and a different driver with a `Send` handle keeps the borrow.

The instrument that says so is in the tree and says it in its own voice
(`crates/happenstance-ladybug/src/live_handle.rs:16-31`):

> Both halves of that are properties of **rusqlite**, not of live handles.
> LadybugDB's `Connection` is `Send` *and* `Sync` … Neither objection survives
> … The evidence for dropping the GAT is therefore one-driver-wide. That does
> not make dropping it wrong; PS-5's *other* argument — that an owned batch
> removes `error[E0195]` and the `where Self: 'a` bound from every impl — is
> untouched by this … It means the clause should rest on that argument rather
> than on this one.

**4. The argument that does hold — `E0195`, and a compiler crash.** Two
transcripts, both reproduced at 1.97.1.

`references/adapter-shapes.md:186-194`:

> Binding an owned type buys **none** of the E0195 relief PS-5 promises: the
> trait method still declares `batch: Self::Batch<'_>`, so writing
> `batch: SqliteBatch` in the impl is `error[E0195]: lifetime parameters or
> bounds on method 'commit' do not match the trait declaration`. The literal
> `Self::Batch<'_>` stays mandatory until the GAT leaves the port itself.

and `:307-365`, the ICE, whose minimisation names five independently necessary
ingredients — *"the trait in a **different crate**; the GAT's **`where Self:
'a`**; an **RPITIT** return; `Self::Batch<'_>` in that method's signature; and a
**non-`'static`** impl self type"* (`:346-350`) — and closes:

> **`where Self: 'a` on the port's GAT is one of the five ingredients**, and
> phase 6 decides whether that GAT survives. If it goes, so does the workspace's
> exposure to this ICE.

`live_handle.rs:60-66` states the consequence in the port's own terms:

> So today's port is implementable **only by stores that outlive every batch
> lifetime**, in practice only by `'static` ones, and the failure mode is an ICE
> rather than a diagnostic. That is a much stronger argument for PS-5 than the
> `Send` one, and it belongs to phase 6.

### The seam nothing could write through

`kb-open-question-projection-batch-no-apply-001` records the hole: *"Generic code
can `begin` a batch and hand it straight to `commit`, and cannot write to it in
between — there is no `apply`"*
(`.kb/open-questions/projection-store-batch-has-no-apply-seam.md:40-42`). Its
sub-question 1 offers three shapes — a bound on `Batch`, a second associated
type, or a method on `ProjectionStore` — and asks which.

**The answer is none of the three**, and flattening it to *"the port grows a
write method"* is the single most likely way to read this ADR wrong. The
reconciliation is a split by consumer, and `spec/SPECIFICATION.md:4934-4945`
already states it:

> An **application's runner** does not need a generic write vocabulary. ADR-0007's
> pump compiles against `projection.rs` unchanged (compiled; PRESSURE-TEST §3.4)
> … The **conformance suite** does need one, because it is generic over an
> adapter it has never seen.

### The sweep's finding, and what it does to this ADR's scope

`references/evaluation/ps-clause-pairing-sweep.md` (2026-08-13, pinned `2136dde`)
swept all 37 `PS` clauses for the coupling-versus-progress pairing defect PS-1
and PS-19 carry. **Verdict: isolated** — three `independent` same-shape defects
against a pre-declared threshold of five, and two of those inside §4.11's table
against a threshold of three. No re-plan is raised and this ADR's scope is not
widened.

Two of its `defective` rows fall inside PS-4 – PS-15 and are therefore this ADR's
to **name and defer**, not to repair:

- **PS-8.** Its `MUST` is about the method's *existence* — *"`rollback` MUST
  remain on the port even though a buffered batch could be dropped"* — while
  `rollback_leaves_both_unchanged` tests its *behaviour*. "Rollback undoes" is
  stated by no clause's `MUST` in §4.
- **PS-13.** Its `MUST` binds a **projection**; `rebuild_is_chunk_size_invariant`
  runs in the **adapter** suite, where the projection is the testkit's own and is
  conformant by construction, so the rule can never observe a PS-13 violation and
  can fail with a conformant projection against a write-behind adapter that PS-4
  and PS-12's second arm both bless.

Both clauses are `[FROZEN]`. The mechanical test settles who owns them: *a
correction to a `[FROZEN]` clause is a repair if the set of implementations the
clause admits is unchanged; otherwise it is a gap, and a gap is a decision's*
(`.kb/decisions/README.md:20-22`). Both corrections change the admitted set, so
both are **gaps**, and a gap is a new decision atom's under
`kb-playbook-repair-frozen-clause-001` — landing at
`unstable-projection-gate-and-clause-disposition`, not here. **This ADR repairs
nothing.**

The sweep also hands this ADR one cross-clause finding that *is* in scope,
because it is about the probe rather than about a clause:
`rebuild_is_chunk_size_invariant` is ungated while
`batch_reads_reflect_pending_writes` carries `READS_THROUGH_BATCH`, so a
write-behind adapter that legitimately declines PS-12's read path cannot pass it.
The probe's design is this ADR's; the rule's gate is the suite stories'.

## Decision

### 1. `type Batch;` — owned, with no lifetime parameter

The port declares an owned associated type. The clause is PS-5, and **it rests on
`E0195` and the ICE, not on `Send`.**

That distinction is the whole of this section. An ADR-0017 that rested PS-5 on
the `Send` argument would have been refuted by an instrument already in the tree
before it was accepted: `crates/happenstance-ladybug/src/live_handle.rs:14-31`
binds a genuinely borrowed, genuinely live handle to `type Batch<'a>` on the
**`Send`** flavour, with real bodies, across a real `tokio::spawn`. The rusqlite
rejection is one driver's `!Sync` `Connection`, generalised past its evidence.

What survives that instrument, and what the clause therefore rests on:

- **`error[E0195]` is unconditional while the GAT is on the port.** Binding an
  owned type to `type Batch<'a>` buys none of the relief; the impl must still
  spell `Self::Batch<'_>` literally (`references/adapter-shapes.md:186-194`).
  This is the trap PS-34 exists for and *"the entire explanation for zero
  adapters"* (`spec/SPECIFICATION.md:5553-5557`).
- **A non-`'static` store crashes the compiler**, in the error-reporting path,
  with `DefId::expect_local` — and `where Self: 'a` on the port's GAT is one of
  the five independently necessary ingredients
  (`references/adapter-shapes.md:307-365`). Today's port is implementable *only
  by stores that outlive every batch lifetime*.

Dropping the lifetime removes both at once, and removes the workspace's exposure
to an open upstream ICE as a side effect rather than as a hope.

**What the owned batch does not buy, stated so nobody claims it later.** It does
**not** close the foreign-batch hazard. Tying the batch to the receiver's
lifetime was compiled and refuted (`spec/SPECIFICATION.md:5108-5116`): *"A
lifetime names a **region** of the program, not an **instance**, and two `&Store`
references unify to a common region without complaint."* The only type-level
construction that names an instance is a generative brand, which forces
`store.with_batch(|batch| …)`, fights `async` at every turn, and forbids the
batch escaping the closure — *"The construction defeats the caller the hazard is
about"* (`:5117-5124`). So **PS-15 stays `[PROVISIONAL]` and is discharged at run
time**: `begin` stamps the batch with a per-store-instance identity and `commit`
compares, returning `CommitError::ForeignBatch`. With the batch owned, the stamp
is a field and the check is an integer comparison on a path already doing I/O.

### 2. No universal write vocabulary — and a probe in the contract crate

**PS-9: `Batch` carries no write bound.** A projection writes through the
concrete adapter's inherent API.

**PS-11: the conformance suite gets its own seam**, because it is the one
consumer that is generic over an adapter it has never seen:

```rust
// the contract crate, behind `feature = "conformance"`.
// Bare flavour only: the suite binds the weaker trait (CLAUDE.md rule 4) and
// the per-test wrapper is a parameter (CF-23), so no `Send` bound is needed
// anywhere.
pub trait ProjectionProbe: ProjectionStore {
    const READS_THROUGH_BATCH: bool;
    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64);
    fn probe_delete_all(&self, batch: &mut Self::Batch);
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error>;
    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
}
```

**These are one decision, not two.** The split is by consumer: the application's
runner never needed the vocabulary, and the suite cannot work without one. Read
as two, the second looks like a concession and invites someone to "simplify" it
back into a bound on `Batch`, which is exactly what PS-9 forbids.

**It lives in `happenstance-core`, not in the testkit, for a coherence reason.**
An adapter crate implementing a testkit trait for its own type is legal — the
type is local — but the natural place to write that impl is the adapter's
`tests/` directory, *"and that is a **different crate**: neither the trait nor the
type is local there, and the impl is rejected"*
(`spec/SPECIFICATION.md:5015-5031`). Putting it in the testkit forces every
adapter into a non-dev dependency on `happenstance-testkit`. Putting it in the
contract crate costs *"one feature flag on a dependency the adapter already has,
and no new edge in the graph."*

`probe_delete_all` exists so the suite can exercise `reset` (PS-16) without
knowing what the read model is. That is ADR-0018's clause, served by this ADR's
mechanism.

**Provisional, with the falsifier and the phase named.** PS-9 and PS-11 are
`[PROVISIONAL]` on the same observation: *a second generic consumer* — any library
code happenstance itself ships that must write into an unknown adapter's batch. A
generic dead-letter recorder and a generic counter projection are the two named
candidates. If either lands, `Batch` grows a bound and PS-9 is replaced; PS-11's
probe becomes redundant in the same motion. **Evaluated at the typed layer's phase
exit (RUNBOOK phase 3 / HS-P0011), by counting consumers.**

### 3. Dropping a batch rolls back, and leaves the store usable

**PS-7, `[FROZEN]`, and both conjuncts are load-bearing.** *"'Rolls back' alone
certifies a store that has permanently lost its only writer"*
(`spec/SPECIFICATION.md:4907-4910`) — a reviewer's probe already found exactly
that: a pooled connection whose `Drop` returned it to nothing, after which the
store answered `Busy` forever.

**PS-8, `[FROZEN]`: `rollback` stays on the port** even though a buffered batch
could be dropped. Rust has no async `Drop`, so an adapter holding a real
transaction has no way to issue `ROLLBACK` and await its completion from a
destructor; removing `rollback` would confine PS-4's *"MAY back one with a live
transaction"* to adapters that can release synchronously. It is also what makes a
fan-out runner's `AssertUnwindSafe` assertion honest rather than a lie, which is
ADR-0019's PS-30.

### 4. `LiveHandleProjectionStore` moves to `experiments/`, and is not deleted

Named explicitly, because Architecture brief AC-A05 and Note 7 require a
disposition and because **deleting the only compiled evidence against the
decision you are making is how a port gets frozen against its own hypothesis.**

Once `type Batch;` lands, `crates/happenstance-ladybug/src/live_handle.rs` cannot
compile against the port: it binds `type Batch<'a>`. Three dispositions were
available — delete it, move it, or keep it with a note — and the decision is to
**move it to `experiments/live-handle-projection-batch/`**, beside
`experiments/rustc-ice-gat-foreign-trait/`, carrying its module documentation and
both transcripts intact, pinned against the pre-freeze port.

The reasoning is the precedent already in the tree. `experiments/` is
*"measurements. reproducible, and not in the gate"* (`CLAUDE.md`), which is
exactly what this module is: a reproducible refutation of an argument the
specification made, that must not be compiled against a port it was never about.
Deletion would leave PS-5 resting on an argument with no surviving counter-case,
and a reader six months from now would find only the conclusion. Keeping it in the
adapter crate would break the gate.

Execution is `owned-batch-port-shape`'s, bounded by project AC-013's *"no change
other than the removal of the batch's lifetime parameter"*.

## Consequences

**For an adapter author.** The `E0195` trap disappears: `async fn commit(&self,
batch: MyBatch, …)` compiles, and there is no `where Self: 'a` clause to write for
a lifetime the `Send` flavour cannot use. A non-`'static` store becomes
implementable, and the workspace stops being able to reach the ICE at
`references/adapter-shapes.md:307-365`. In exchange the author implements
`ProjectionProbe` behind one feature flag on a crate they already depend on — and
by `CLAUDE.md`'s rule, an adapter that does not implement it cannot invoke the
suite and therefore does not exist.

**For a store that genuinely wants a live borrowed handle.** It buffers, or it
owns. LadybugDB's handle is `Send` and `Sync` and could have kept the borrow, and
this ADR takes it away from it anyway — which is why the disposition of §4 is
recorded rather than assumed. PS-34 dies the moment `type Batch;` lands; if PS-5
is ever falsified, PS-34 becomes binding again and the doctest it names is owed.

**For the suite.** `ProjectionProbe` is the mechanism, not a rule of its own
(`spec/SPECIFICATION.md:4981-4983`). Every projection rule is downstream of it,
and `READS_THROUGH_BATCH` is a probe const rather than a fixture capability
because PS-12's decline is a property of the batch type, not of the fixture's
environment.

**For the feature surface.** One new off-by-default feature, `conformance`, on
`happenstance-core`, which `cargo hack --feature-powerset` already exercises. It
sits beside `unstable-projection` (PS-3), and the interaction of the two is
`projection-probe-conformance-feature`'s to land and
`publication-and-positioning`'s to expose or not.

**What stays open.** PS-15 is discharged at run time, not in the type system, and
that is a smaller promise than the clause's reader may expect. PS-9 and PS-11 are
provisional on a consumer count nobody can observe passively — the same shape
ADR-0007's own falsifier has, and the same risk: *"nobody ever needed it" is not
something you can observe by waiting* (`spec/SPECIFICATION.md:4812`). The
phase-3 exit criterion that counts consumers is what makes these markers mean
anything.

**One correction this ADR owes and does not make.** PS-32 requires ADR-0007's
Context to be corrected: a callback-driven pump *can* be written against the port
as it stands; what cannot be written is the conformance suite. ADR-0007 is an
**accepted, immutable** decision atom, so the correction is a superseding atom's
and never an edit (`.kb/decisions/README.md`). This ADR records the correction as
owed and states its shape; it does not perform it.

## Alternatives rejected

**A `ProjectionBatch` supertrait carrying `fn put(&mut self, key: &str, value:
&[u8])`.** The obvious answer to "nothing can write to the batch", and it looks
free. It is not: *"it obliges a graph store and a relational store each to carry a
key-value table nobody asked for, and it reintroduces at the read-model layer the
opaque blob that ADR-0003 deliberately confined to event payloads. A read model
exists to be queried by the application; a blob keyed by string is not one"*
(`spec/SPECIFICATION.md:4957-4962`). Rejected because it pays a permanent
modelling cost for one consumer's convenience — and that consumer is the suite,
which §2 serves without it.

**The probe trait in `happenstance-testkit`.** Rejected by the orphan rule, and
expensively: the impl's natural home is the adapter's `tests/` directory, a
different crate where neither trait nor type is local
(`spec/SPECIFICATION.md:5015-5031`). The cost of the rejection is a non-dev
dependency on the testkit for every adapter, and a new edge in the dependency
graph that publication then has to justify.

**Keeping the GAT.** Defensible on the evidence for exactly one driver family —
`GraphWriteHandle<'a>` compiles on the `Send` flavour — and refuted by the two
arguments that are driver-independent: `E0195` on every impl, and a compiler crash
for any non-`'static` store. The GAT's benefit accrues to stores that already have
a `Send + Sync` handle; its cost is paid by every implementer.

**Resting PS-5 on the `Send` argument.** Rejected as *already refuted in this
workspace*. Recorded as a rejected alternative rather than silently avoided,
because §4.2 of the specification still makes it and a reader who starts there
will reach for it (`crates/happenstance-ladybug/src/live_handle.rs:9-31`).

**A generative brand for the foreign-batch hazard.** Works, and is unusable:
`begin` becomes `store.with_batch(|batch| …)`, which fights `async` and forbids
the batch escaping the closure — precisely what a runner holding a
`HashMap<DepotId, Store>` needs it to do (`spec/SPECIFICATION.md:5117-5124`).
Rejected in favour of a run-time stamp, with PS-15 left `[PROVISIONAL]` so a
zero-cost construction can still replace it with a compile error, which would be
strictly better.

**Tying the batch to the receiver's lifetime.** E2E-19 called it *"the cheapest fix
in the entire catalogue"*. It was compiled and it does not work — a lifetime names
a region, not an instance. Kept in this list because it is the fix every reader
reaches for first.

**Deleting `LiveHandleProjectionStore`.** Rejected in §4. It is the only compiled
evidence against this ADR's own §1.
