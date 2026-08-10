---
id: question-projection-store-port-is-provisional
title: The `ProjectionStore` port has no conformance suite, so its shape is a guess
kind: open_question
status: draft
authority_tier: note
summary: >-
  `grep -rn "ProjectionStore for"` matches nothing in this workspace, so the port is
  currently shaped like nothing at all. A port without a conformance suite is a guess. It
  gets frozen when the first real projection adapter can be built against it — phase 6 —
  and the bar is `CheckpointOnlyStore` FAILING the suite.
depends_on: []
related:
  - concept-the-batch-shape-axis
  - playbook-freezing-a-port
  - adr-0007-projection-runner-decodes
source_paths:
  - crates/happenstance-core/src/projection.rs
  - docs/architecture/SPECIFICATION.md
last_reviewed: 2026-08-09
---

# The projection store port is provisional

**What is true today.** The port exists in `crates/happenstance-core/src/projection.rs` with
a GAT-shaped `Batch<'a>`. Nothing in the workspace implements it —
`grep -rn "ProjectionStore for"` matches nothing. There is no conformance suite for it, and
§4's clauses are marked `[PROVISIONAL]` accordingly. The invariant the port exists for is
documented on the trait: the read-model write and the checkpoint write land in one
transaction.

**What is not decided.** Whether the batch keeps its lifetime parameter; what vocabulary
writes into it; what a dropped batch owes; how a projection is returned to "never run";
whether `commit`'s position may regress; whether a batch is read-your-writes within a chunk;
and what happens when `apply` fails. Also whether the port ships at 0.1 at all, or behind an
off-by-default `unstable-projection` feature with a documented semver exemption — the
`tokio_unstable` idiom.

**What forces it.** Phase 6. Its bar is not "the suite passes": it is `CheckpointOnlyStore`
— a store that commits the checkpoint and silently drops the read-model write — **failing**
at least one named rule, and two structurally unlike batch shapes passing.

**Ordered sub-questions.**

1. Does the batch keep a lifetime? (ADR-0017)
2. What does the write seam look like, given that generic suite code holding a `P::Batch`
   can otherwise only pass it to `commit` or `rollback`?
3. How is a projection reset, and what may refuse it? (ADR-0018)
4. What is the failure policy, per projection rather than per runner? (ADR-0019)
5. Does the port ship at 0.1, or behind a feature?
