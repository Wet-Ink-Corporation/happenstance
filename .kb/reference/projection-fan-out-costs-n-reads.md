---
id: kb-reference-projection-fan-out-cost-001
title: What &mut P fixes about fan-out — N projections cost N reads
kind: reference
status: accepted
authority_tier: note
summary: >-
  Recorded 2026-08-16 against commit 78a2170, from phase 7's use of the
  frozen contract. Because apply takes &mut P, a runner cannot drive more
  than one projection from a single replay: N projections cost N reads of
  the same events, and the cost is fixed at the API level rather than by any
  adapter's implementation. This is a shape and not a wrong answer. The
  runner's own defence is on the record and is sound for the alpha — an
  owned write set cannot be shared between projections, and one failure
  policy for every projection would be wrong, which is ADR-0019's
  per-projection policy decision. The projection port family is PROVISIONAL
  and has no conformance suite, and ADR-0019 already names the open half:
  PS-30 falls if the fan-out runner is never built, decided by whether the
  poll cost of N independent reads is real, which is a benchmark this
  workspace has no harness for. This atom supplies the half that needs no
  harness — the arity is derivable from the signature — so that whichever
  decision eventually meets the tail seam meets a figure already on the
  record rather than starting from an argument. It supplies no measurement
  of what a read costs, and none should be inferred from it.
depends_on: []
related:
  - kb-decision-0019
  - kb-decision-0017
  - kb-open-question-read-page-budget-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-contract-defects.md
  - crates/happenstance/src/runner.rs
  - crates/happenstance-core/src/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
---

# What &mut P fixes about fan-out — N projections cost N reads

## What this is a pointer to

Phase 7's defect log (finding C5) traced a cost through the projection port's
signature rather than through any adapter's code, and the trace is derivable
from the type alone: `ProjectionStore::apply` takes `&mut P`. An exclusive
reference admits exactly one caller, so a single replay of an event stream
cannot drive two projections at once. Running a second projection over the
same events costs a second, independent read — the cost is fixed by the API
shape, not by how any particular store chooses to implement it. This atom
records that derivation as a citable figure, distinct from the open question
of whether the cost matters in practice.

## The finding

**N projections cost N reads, at the API level.** `&mut P` in `apply`'s
signature (`crates/happenstance-core/src/projection.rs`) means a runner
holding one mutable reference to a projection cannot simultaneously hold one
to a second — the borrow checker forecloses the shared-replay shape before
any adapter is written. `crates/happenstance/src/runner.rs` drives each
projection with its own read of the stream rather than fanning one read out
to many, and that is not an implementation choice the runner made; it is the
only shape `&mut P` admits.

**This is a shape, not a wrong answer.** The runner's own documented defence
holds for the alpha: an owned write set cannot be shared between two
projections without introducing shared mutable state the port was designed
to avoid, and collapsing every projection onto one failure policy would be
wrong — `kb-decision-0019` decided per-projection failure policy for exactly
this reason, because different projections legitimately want different
answers to "what happens when apply fails."

## Why this is filed as reference rather than as the open question itself

The projection port family is `[PROVISIONAL]` in `spec/SPECIFICATION.md` and
has no conformance suite yet — a port without one is a guess, per this
repository's own standard. `kb-decision-0019` already names the tail-seam
question directly: PS-30 falls if a fan-out runner sharing one replay across
projections is never built, and that choice turns on whether the poll cost
of N independent reads is real enough to justify the shared-state complexity
a fan-out runner would introduce. Answering that requires a benchmark this
workspace has no harness for — a measurement of what a read actually costs
against a real store, which nothing here provides.

What *is* available without a harness is the arity itself: it follows from
the signature, not from timing anything. This atom supplies exactly that
half, so that whichever future decision meets the tail-seam question starts
from a figure already on the record — N reads for N projections — rather
than re-deriving it from an argument. It supplies no measurement of what a
single read costs against any adapter, and none should be inferred from the
arity alone; a small N and an expensive read might justify a fan-out runner
even at this arity, and a large N and a cheap read might not.

## Provenance

Recorded 2026-08-16 against commit `78a2170`, from phase 7's `contract-defect-log`
(finding C5, `references/evaluation/phase-7-contract-defects.md`). The
signature cited is `crates/happenstance-core/src/projection.rs`'s
`ProjectionStore::apply`; the runner behaviour cited is
`crates/happenstance/src/runner.rs` as it stood at that commit.
`kb-decision-0019` is the accepted record of the per-projection failure
policy this atom's defence paragraph rests on; `kb-decision-0017` is the
accompanying record of what a projection batch owns.
