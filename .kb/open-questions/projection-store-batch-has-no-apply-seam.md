---
id: kb-open-question-projection-batch-no-apply-001
title: ProjectionStore::Batch carries no bounds, so nothing can write to it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ProjectionStore::Batch is an associated type with no trait bounds, so generic code can begin a
  batch and commit it and cannot write anything into it — the port has no apply. ADR-0007 records
  this and declines to settle it, and it is the reason ADR-0006's relocation of the projection
  runner went unchallenged for a day: nothing had tried to compile against the port. What is not
  decided is what vocabulary writes into a batch — a bound on the associated type, a second
  associated type, or a method on the port — and the answer is entangled with ADR-0008's finding
  that a provided body cannot hold the Batch GAT across a suspension point under any remedy tried.
  Owned by phase 6, which freezes ProjectionStore and carries ADR-0017 for what a projection batch
  owns and what vocabulary writes into it. Forced by the first real projection adapter: the port
  has no conformance suite, and a port without one is a guess.
depends_on: []
related:
  - kb-decision-0007
  - kb-decision-0008
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-open-question-projection-id-unvalidated-001
source_paths:
  - .kb/_intake/0007-projection-runner-decodes.md
  - references/adr/0007-projection-runner-decodes.md
  - crates/happenstance-core/src/projection.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# ProjectionStore::Batch carries no bounds, so nothing can write to it

## What is true today

ADR-0007 (`.kb/decisions/0007-projection-runner-decodes.md`) records a "second discovery" made while
testing ADR-0006's claim that the projection runner belongs wholly in `happenstance-core`:
`ProjectionStore::Batch` carries no trait bounds. Generic code can `begin` a batch and hand it
straight to `commit`, and cannot write to it in between — there is no `apply`. The runner
ADR-0006 relocated cannot be written against the port as it stands, in either crate.

The ADR is explicit about why this matters beyond the runner's own placement: "it is recorded
here because it is why the relocation went unchallenged. Nothing tried to compile against it."
ADR-0006 moved the projection runner into `happenstance-core` on the strength of a discriminator
— encoding versus orchestration — that was asserted rather than tested, because no runner existed
when it was written. The missing `apply` seam is what made that possible: a decision about where
code lives, taken before the code exists, went unchallenged for a day because nothing forced a
compile against the port it was made about.

ADR-0007 itself does not close the gap. It splits the runner at the decode boundary — a
`happenstance-core` pump that never decodes, and a `happenstance` `Projection` trait an
application implements — and its indicative `Projection::apply` signature takes
`&mut <Self::Store as ProjectionStore>::Batch<'_>` as a parameter. But the signature is
illustrative ("names are the implementation's to settle") and does not itself add a bound to
`ProjectionStore::Batch`; the ADR's own consequences section states plainly: "the port still needs
its apply seam, and this ADR does not settle its shape — that is phase 2's, with its own ledger
row."

## What is not decided

What vocabulary a caller uses to write into a batch. Three shapes were visible at the time ADR-0007
was written and none was chosen: a trait bound on `Batch` itself (so any batch exposes some
`write`-shaped method), a second associated type carrying the write surface, or a method on
`ProjectionStore` that takes the batch and a payload directly rather than routing through the
batch type. The choice is entangled with a separate finding attributed to ADR-0008 — that a
provided default-body method cannot hold the `Batch` GAT across a suspension point under any
remedy that was tried — which constrains which of the three shapes can actually compile against
an async projection store.

## What forces it

Phase 6, "Freeze `ProjectionStore`", which is also where `RUNBOOK.md` schedules ADR-0017. The
projection store port cannot be frozen honestly while it has no apply seam: freezing a port means
committing adapters to its shape, and a port that cannot express writing an event into a batch has
not yet stated what an adapter must implement. More immediately, the port has no conformance
suite — CLAUDE.md states this directly: "It gets frozen when the first real projection adapter can
be built against it." The first real projection adapter is exactly the thing that cannot be built
until this question is answered, because it is the adapter that would need to call `apply`.

## Ordered sub-questions

1. Does the apply seam belong on `Batch` as a bound, on `ProjectionStore` as a method, or as a
   second associated type — and does ADR-0008's GAT-across-a-suspension-point finding rule out any
   of the three before phase 6 even starts weighing them?
2. Once a shape is chosen, does `Projection::apply`'s indicative signature in ADR-0007 survive as
   written, or does the parameter type it names change shape along with the port?
3. Does closing this question retroactively validate or invalidate ADR-0006's discriminator —
   encoding versus orchestration — as the right line to have split the runner on, now that the
   thing that made it untestable is gone?
4. Who writes the projection conformance suite once the seam exists, and does it reuse
   `happenstance-testkit`'s existing rule-and-fixture machinery or need its own?
