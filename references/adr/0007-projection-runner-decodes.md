# ADR-0007: The projection runner decodes, and therefore splits across the seam

- **Status:** accepted
- **Date:** 2026-08-06
- **Partly supersedes:** [ADR-0006](0006-bare-name-to-the-typed-layer.md)

## Context

[ADR-0006](0006-bare-name-to-the-typed-layer.md) allocated the bare name to the
typed layer, and — in the same decision — moved the projection runner into the
contract crate. Its discriminator was encoding:

> *"The discriminator for what belongs in the typed layer is **encoding**, not
> orchestration. The projection runner pumps an `EventStore` into a
> `ProjectionStore` and never decodes a payload."*

The discriminator is right. The claim about the runner is not, and it was
asserted rather than tested — no runner existed when it was written.

Testing it against the consumer it is for: an application takes SQLite as its
event store and keeps read models in **both** SQLite and LadybugDB. It has
several materialised views. Each view nominates the events it cares about, and
the application shapes the record — whatever calculations that view needs. What
it wants from happenstance is a surface that feeds it events in order,
**decoded**, and wires up the projection store.

That expectation is not exotic; it is the ordinary one. And a runner that never
decodes cannot meet it. It would orchestrate the checkpoint and hand back opaque
[`Bytes`](https://docs.rs/bytes) — which is the half nobody needed help with.
The application would still write the decode, per view, by hand. So the runner
as ADR-0006 scoped it supplies the easy half of a two-part job and calls itself a
convenience.

A second discovery in the same pass compounds it: `ProjectionStore::Batch`
carries **no trait bounds**, so generic code can `begin` a batch and hand it back
to `commit`, and cannot write to it. There is no `apply`. The runner ADR-0006
relocated therefore cannot be written against the port as it stands — in either
crate. **[CORRECTED 2026-08-17 by ADR-0031 — see "Correction" at the end of this record.]** That is a separate question about the port, tracked in the decision ledger
and owned by phase 2; it is recorded here because it is why the relocation went
unchallenged. Nothing tried to compile against it.

## Decision

Split the runner at the decode boundary, because it is two things and only one of
them is about encoding.

| Crate | Holds |
|---|---|
| **`happenstance-core`** | The checkpoint pump. Reads from the checkpoint, chunks, `begin`s, invokes a callback per `SequencedEvent`, `commit`s with the last position applied. Never decodes. |
| **`happenstance`** | The `Projection` trait and the runner an application actually uses: decoded events, the projection's `Query`, the store's `Batch`. |

ADR-0006's discriminator survives intact. What changes is the claim that the
runner falls wholly on the core side of it. It falls on both sides, because the
transactional invariant and the decode are different jobs — and the invariant
belongs in the crate that defines it, beside the port whose module documentation
argues for it.

Indicative shape; names are the implementation's to settle:

```rust
// happenstance-core — the invariant, no encoding.
pub async fn pump<E, P, F>(events: &E, store: &P, id: &ProjectionId, query: &Query, apply: F)
    -> Result<Option<SequencePosition>, PumpError<E::Error, P::Error>>
where
    E: EventStore,
    P: ProjectionStore,
    F: FnMut(&mut P::Batch<'_>, &SequencedEvent) -> Result<(), P::Error>;

// happenstance — what an application implements.
pub trait Projection {
    type Store: ProjectionStore;
    type Event: DomainEvent;

    fn id(&self) -> ProjectionId;
    fn query(&self) -> Query;
    fn apply(
        &mut self,
        batch: &mut <Self::Store as ProjectionStore>::Batch<'_>,
        event: Sequenced<Self::Event>,
    ) -> Result<(), Self::Error>;
}
```

Three shape decisions ride along, because the narrative above settles them and
leaving them implicit is how they get settled wrongly:

**A projection nominates its events with `Query`.** The same type a decision
model uses. There is no second filtering vocabulary, and a projection's
subscription is checkable against the same semantics the conformance suite
already pins. This is DCB's query doing double duty, and it is free.

**`Projection::Store` is an associated type, so a projection targets exactly one
store.** A projection spanning SQLite *and* Ladybug is then unrepresentable —
which is correct, because there is no cross-store transaction and one that
appeared to work would be lying about the invariant
[`ProjectionStore`](../../crates/happenstance-core/src/projection.rs) exists to
defend. This is the crate's own "illegal states are unrepresentable" applied at
the point it is most likely to be got wrong.

**Checkpoints stay per `(store, ProjectionId)`**, so views advance and rebuild
independently. It follows that two stores projecting the same events sit at
different positions at any given moment. That skew is inherent, not a defect, and
callers reading both must tolerate it — so it is documented rather than left to
be discovered.

## Consequences

**Good.** The convenience lands where the consumer meets it. `happenstance-core`
keeps the checkpoint invariant beside the port that states it, so the crate an
adapter author pins still owns the rule an adapter must honour.

**Good.** Reusing `Query` means a projection's subscription inherits query
semantics that are already conformance-tested, rather than growing a parallel
filter language that is not.

**Good.** Making `Store` an associated type converts a documentation problem —
"do not write a projection across two stores" — into a compile error.

**Bad.** Two runners to document, and the core pump may have no caller but the
typed one. **Falsification:** if it has acquired no independent caller by the
time the typed layer's phase exits, collapse it upward and supersede this ADR.
Do not keep a seam because it was argued for.

**Bad.** ADR-0006 is amended the day after it was accepted — the same churn it
criticised in ADR-0005, and the second time a decision made in one sitting has
needed correcting once something was built against it. The mitigation is the same
one ADR-0006 used: record why. The pattern is now legible enough to name — a
decision about where code lives, taken before that code exists, is a guess. The
provisional marker on ADRs 0001, 0003 and 0004 exists for exactly this, and
ADR-0006 exempted itself from it on the grounds that a naming decision is settled
by being made. That was true of the *name*. It was not true of the runner
allocation bundled with it, which is the third time an "and" in an ADR title has
concealed a second, weaker decision.

**Neutral.** The port still needs its apply seam, and this ADR does not settle
its shape — that is phase 2's, with its own ledger row. This ADR settles only
where the runner lives and what it hands the application.

## Alternatives rejected

- **Leave the runner entirely in `happenstance-core`; the application decodes
  inside `apply`.** Cheapest, and ADR-0006 stands verbatim. Rejected because it
  supplies the orchestration nobody found hard and withholds the decode everybody
  writes — every application would hand-roll the same wrapper, which is the
  definition of a missing library function.

- **Move the whole runner into `happenstance`.** Simpler: one runner, one place,
  no seam to justify. Rejected, but only narrowly — it puts the checkpoint
  invariant, the single thing the projection port exists to defend, in a crate
  that sits above the port and can be swapped out. Keeping the invariant beside
  its definition is worth one thin seam. This is the fallback if the
  falsification above fires.

- **A projection store that decodes.** Rejected on ADR-0003's reasoning, which is
  unaffected: a store that parses payloads is a store carrying domain knowledge,
  and the adapter population is exactly who must not have any.

## Note on the historical record

ADR-0006's body is kept **verbatim**, per the rule it applied to ADR-0005 and
ADR-0005 applied to ADR-0002. Its naming decision — the bare name to the typed
layer, the contract to `happenstance-core` — is untouched and still stands on its
own reasoning; only the runner allocation moves. The rename it mandates remains
unexecuted at the time of writing, so the crate names used above are the ones
ADR-0006 specifies, not the ones on disk today.

## Correction — 2026-08-17, by ADR-0031

**Appended, not rewritten, and appended *below* every cited line range on purpose.**
`spec/SPECIFICATION.md` cites this record at `:44-50`, `:62-67` and `:76-81`, and two
backlog specs cite `:62-67` and `:118-121`; `cargo xtask spec-trace` is a gate step. An
insertion anywhere above those anchors would silently re-point all of them, so the marker
at `:38` was written into an existing line and the substance lives here.

**The claim at `:36-38` is false.** It reads: *"The runner ADR-0006 relocated therefore
cannot be written against the port as it stands — in either crate."*

**What is true:** a callback-driven pump **can** be written against `ProjectionStore` as it
stands, because the closure's caller — not the port — knows the concrete `Batch`. What
could not be written against the port as it stood is the **conformance suite**.
`spec/SPECIFICATION.md` §4.9 states this as `[FROZEN]` clause **PS-32**, and §4.3 is where
the runner/suite distinction lives.

**It was falsified by compilation, not by argument.**
`references/evaluation/PRESSURE-TEST.md` section 3.4 builds this record's own indicative
pump — `checkpoint` → `begin` → per-event `apply` → `commit`/`rollback` — against
`crates/happenstance-testkit/src/projection.rs` unchanged. The generic-code claim in the
same sentence was never tested; the pump was.

**The correction of record is `kb-decision-0031`**
(`.kb/decisions/0031-the-runner-collapses-upward.md`), "One runner, in `happenstance` — the
checkpoint pump collapses upward", which carries the corrected Context in its own body. Per
`.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59`, correcting a
load-bearing factual claim is a superseding decision's act and never a hand-edit to the
reasoning — so the paragraph above is left exactly as it was written, and this section
states what is now known instead of pretending the record always said it.

**The supersession is partial.** ADR-0031 supersedes only this record's allocation of a
checkpoint pump to `happenstance-core`. The discriminator — encoding, not orchestration —
and all three shape decisions that rode with the split (`Query` as the only nomination
vocabulary, `Projection::Store` as an associated type, checkpoints per
`(store, ProjectionId)`) survive untouched and implemented as written. `kb-decision-0007`
therefore stays `accepted` with `superseded_by: null`, and `kb-decision-0030` still
depends on one of the three.

**No pump was ever written, and none will be.** `happenstance::run_projection` drives the
port directly and `happenstance-core` publishes no module-level function that runs — held
by an executed test at `crates/happenstance/tests/projection_clauses.rs`. That is the
falsification condition at `:118-121` firing exactly as this record specified it.
