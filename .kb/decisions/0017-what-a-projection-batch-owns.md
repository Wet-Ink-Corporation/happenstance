---
id: kb-decision-0017
title: What a projection batch owns, and the seam that is not a write vocabulary
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0017
reversibility: low
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with two halves provisional and both falsifiers named: PS-9/PS-11 falls if a second
  generic consumer appears — a dead-letter recorder or counter projection happenstance itself
  ships — counted at the typed-layer phase exit; PS-15 falls to a zero-cost type-level
  construction that names an instance, composes with async fn and still permits a batch in a
  collection, at which point the clause is replaced by a compile error. ProjectionStore::Batch
  becomes an owned type Batch with no lifetime parameter, resting on two compiler transcripts
  rather than on the Send argument — error[E0195] on every impl, and a DefId::expect_local ICE
  proving today's GAT port is implementable only by stores that outlive every batch. The Send
  argument is refuted in this workspace, since LadybugDB's Connection is Send and Sync and a
  genuinely borrowed handle would have bound to a GAT under the Send flavour. An owned batch does
  not close the foreign-batch hazard, because a lifetime names a region and not an instance, so
  PS-15 stays provisional and is discharged at run time by a per-store-instance stamp surfacing
  as CommitError::ForeignBatch. The write seam is split by consumer rather than universalised —
  no write vocabulary on Batch, and a ProjectionProbe: ProjectionStore in the contract crate
  behind feature = "conformance", bare flavour only, because an adapter's tests/ is a third crate
  where neither a testkit trait nor the adapter's type is local and the orphan rule rejects the
  impl. Dropping a batch rolls back and leaves the store usable, and rollback stays on the port
  because Rust has no async Drop. LiveHandleProjectionStore moves to experiments/ rather than
  being deleted — it is the only compiled evidence against this decision's own first claim.
  Rejected: a ProjectionBatch supertrait carrying put/get, which obliges every store into a
  key-value table and reintroduces at the read-model layer the opaque blob ADR-0003 confined to
  payloads; the probe trait in happenstance-testkit; keeping the GAT; a generative brand, which
  works and forbids the batch escaping the closure the hazard is about; and tying the batch to
  the receiver's lifetime, compiled and refuted. Answers the projection-batch apply-seam
  question's sub-questions 1, 2 and 4, leaving sub-question 3 with the typed layer. Names PS-8's
  and PS-13's clause/rule pairing defects as gaps rather than repairs, and repairs neither.
depends_on:
  - kb-decision-0007
  - kb-decision-0008
related:
  - kb-decision-0003
  - kb-decision-0010
  - kb-reference-port-traits-compiled-findings-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-playbook-one-decision-per-adr-title-001
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/2026-08-13-adr-0017-projection-batch.md
  - references/adr/0017-what-a-projection-batch-owns.md
  - references/adapter-shapes.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - crates/happenstance-ladybug/src/live_handle.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-13
---

# What a projection batch owns, and the seam that is not a write vocabulary

## Decision

`ProjectionStore::Batch` becomes an owned associated type, `type Batch;`, with no lifetime
parameter. The clause rests on two compiler transcripts, not on a `Send` argument. The GAT form
fails `error[E0195]` on every impl attempted (`references/adapter-shapes.md:186-194`), and a
second transcript — a `DefId::expect_local` internal compiler error — proves the GAT port is
implementable only by stores that outlive every batch it hands out
(`references/adapter-shapes.md:307-365`; `crates/happenstance-ladybug/src/live_handle.rs:36-66`).
The `Send` argument that a first pass reached for is refuted directly in this workspace:
LadybugDB's `Connection` is both `Send` and `Sync`, so a genuinely borrowed
`GraphWriteHandle<'a>` binds to a GAT `type Batch<'a>` under the `Send` flavour with real bodies —
`Send`-ness was never the obstacle.

An owned batch does not, by itself, close the foreign-batch hazard — the risk that a batch begun
against one store instance is committed against another. A lifetime names a region of code, not a
runtime instance, so no lifetime parameter, borrowed or owned, can rule that out at compile time.
PS-15 therefore stays `[PROVISIONAL]` and is discharged at run time instead: a per-store-instance
stamp compared on `commit`, surfacing as `CommitError::ForeignBatch`.

The write seam is split by consumer rather than given a universal vocabulary on `Batch`. No
`put`/`get`-shaped methods land on the trait. Instead, a `ProjectionProbe: ProjectionStore` lives
in the contract crate itself, gated behind `feature = "conformance"`, bare flavour only. The
placement is forced by coherence, not preference: an adapter's own `tests/` directory is a
separate crate from the adapter, so neither a hypothetical testkit trait nor the adapter's own
type is local to it, and the orphan rule rejects an impl attempted there
(`spec/SPECIFICATION.md:5015-5031`).

Dropping a batch rolls back and leaves the store usable; `rollback` stays a port method rather
than living only in `Drop`, because Rust has no async `Drop` and a store's rollback may need to be
async. `LiveHandleProjectionStore` is not deleted — it moves to
`experiments/live-handle-projection-batch/`, because it is the only compiled evidence against this
decision's own first claim, and `experiments/` is this workspace's home for a reproducible
measurement kept outside the gate.

## Provisional

PS-9/PS-11 (the split-by-consumer write seam) falls if a second generic consumer appears — library
code happenstance itself ships that must write into an unknown adapter's batch, such as a generic
dead-letter recorder or counter projection — counted at the typed-layer phase exit (HS-P0011).
PS-15 (the foreign-batch hazard) falls to a zero-cost type-level construction that names a specific
instance, composes with `async fn`, and still permits a batch to sit in a collection; if found, the
runtime stamp is replaced by a compile error, which is strictly better.

## Alternatives rejected

A `ProjectionBatch` supertrait carrying `put`/`get` obliges every store into a key-value table and
reintroduces, at the read-model layer, the opaque-blob shape ADR-0003 confined to event payloads.
The probe trait was also tried inside `happenstance-testkit`, and rejected by the same orphan-rule
argument that placed it in the contract crate instead. Keeping the GAT was defensible for one
driver family and refuted by two driver-independent costs: `E0195` on every impl, and an ICE for
any non-`'static` store. Resting PS-5 on the `Send` argument was tried and is already refuted in
this workspace. A generative brand for the foreign-batch hazard works and was rejected because it
forbids the batch from escaping the closure that begins it — defeating the caller the hazard exists
to protect. Tying the batch to the receiver's lifetime was compiled and refuted directly.

## Scope and what is left open

This decision answers sub-questions 1, 2 and 4 of
`kb-open-question-projection-batch-no-apply-001`; sub-question 3 — whether closing it retroactively
validates ADR-0006's encoding-versus-orchestration discriminator — stays with the runner
(HS-P0011). The clause-pairing sweep's `defective` findings for PS-8 and PS-13 sit inside this
clause range and are named here as gaps, not repaired: both are `[FROZEN]`, and a correction that
changes the admitted implementation set belongs to a decision atom of its own.
