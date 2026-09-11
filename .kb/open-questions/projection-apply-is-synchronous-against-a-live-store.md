---
id: kb-open-question-apply-synchronous-live-store-001
title: Projection::apply is synchronous, so the runner cannot drive the live store the port was frozen against
kind: open_question
status: accepted
authority_tier: note
summary: >-
  At 86a410c, Projection::apply(&mut self, event: Self::Event) is synchronous
  (crates/happenstance/src/domain.rs:249) and run_projection
  (crates/happenstance/src/runner.rs) folds events through it before handing
  the batch to the store; a projection that writes rows can push into an
  owned buffered batch and cannot await a statement into a live transaction.
  ADR-0062 moved the port's probe seam and begin to async and built
  LivePostgresProjectionStore against a live sqlx transaction; ADR-0063 froze
  the port on that evidence and deliberately kept happenstance's
  unstable-projection gate on the runner alone, because freezing apply now
  would freeze a shape proved at one end of its axis — the mistake ADR-0060
  refused for the port. Both records defer the question by name and neither
  owns it. What is not decided: whether apply moves (to async, to a batch-handle
  parameter, or not at all — the live store may be an instrument for the
  port's freeze and never a runner target, which is ADR-0062's own framing),
  and whether the runner's gate comes off with it. What forces it: the first
  projection an application needs to run against a live-transaction store
  through the runner rather than the probe, or a decision to publish the
  runner ungated. Ordered sub-questions: does apply need to change at all; if
  it does, does the moved batch shape (&mut Self::Batch, async at the probe
  seam) become apply's parameter; and does ADR-0063's falsifier — apply
  moving to a shape that requires the port to move with it — fire on any of
  those answers. The runner gate "no longer forwarding to the contract crate"
  is the ADR-0063 brief's statement about the lane; at 86a410c
  crates/happenstance/Cargo.toml:142 still forwards.
depends_on: []
related:
  - kb-decision-0062
  - kb-decision-0063
  - kb-decision-0060
  - kb-decision-0017
  - kb-decision-0007
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-projection-runner-chunk-observation-001
  - kb-open-question-provisional-falsifiers-001
source_paths:
  - .kb/_intake/2026-09-10-adr-0062-the-probe-seam-moves.md
  - .kb/_intake/2026-09-11-adr-0063-the-projection-port-is-frozen.md
  - crates/happenstance/src/domain.rs
  - crates/happenstance/src/runner.rs
  - crates/happenstance/Cargo.toml
last_reviewed: 2026-09-11
---

# Projection::apply is synchronous, so the runner cannot drive the live store the port was frozen against

## Where the shapes disagree

`Projection::apply` is `fn apply(&mut self, event: Self::Event)` —
synchronous, taking `&mut self` (`crates/happenstance/src/domain.rs:249`).
`run_projection` (`crates/happenstance/src/runner.rs`) folds a batch of
events through `apply` in memory and only then hands the accumulated state
to the store. That shape is exactly right for a buffered
`ProjectionStore::Batch` — an owned value the projection can mutate and the
runner can commit once at the end — and exactly wrong for a batch that
*is* a live transaction: writing a row inside `apply` would need to issue a
statement, which needs `.await`, which a synchronous `&mut self` method
cannot do.

ADR-0062 gave the port a live-transaction end to be wrong against on
purpose: `probe_write`, `probe_delete_all` and `probe_read_through` moved to
`&mut Self::Batch` returning `impl Future`, `begin` moved to `async fn`, and
`LivePostgresProjectionStore` now owns a `sqlx::Transaction<'static,
Postgres>` as its `Batch` and ran 20 of 20 against a live PostgreSQL. ADR-0062
names the resulting gap explicitly rather than closing it: *"the typed
layer's `Projection::apply` is synchronous, so an application can push into a
buffered batch and cannot issue a statement into a live one. The live store
is an instrument for the port's freeze; whether `apply` moves is the typed
layer's axis, not this port's."*

## What ADR-0063 did with that framing

ADR-0063 froze `ProjectionStore`, `ProjectionProbe` and their value types —
a signature change to any of them is now a breaking change with a decision
record behind it, the same status `EventStore` holds. It deliberately did
**not** extend that freeze to `Projection::apply`, and gave the reason in
the same terms ADR-0062 used: *"freezing `apply` now would freeze a shape
proved at one end of its axis, which is the mistake ADR-0060 refused for the
port."* `happenstance`'s `unstable-projection` feature stays, narrowed to
gate the runner alone, for exactly this reason — the runner is the thing
that calls `apply`, and `apply` is unproved against the live end the port
now supports.

## What that leaves unresolved

Three answers, still open, in the order they'd have to be settled:

1. **Does `apply` need to change at all?** A live-transaction consumer could
   in principle stage rows in memory during `apply` and issue them in a
   separate async step the runner adds around it, leaving `apply` as-is. Or
   it may not — ADR-0062's own framing left open whether the live store is a
   port-freeze instrument that no runner projection ever targets directly.
2. **If it does change, what does it become?** The candidate shape mirrors
   the seam ADR-0062 already moved: `async fn apply(&mut self, batch: &mut
   Self::Batch, event: Self::Event)`, trading the buffered `&mut self`
   ergonomics for a shape that can hold a live transaction across an
   `.await`, the same trade ADR-0062 made at the probe seam.
3. **Does the runner's gate come off with it, or separately?** ADR-0063 kept
   the two gates (contract-crate feature, now empty; typed-layer feature,
   narrowed) distinct on purpose. A resolved `apply` shape does not by
   itself answer whether the runner ships stable before, with, or after that
   resolution.

## What forces this

The first application that needs to run a row-writing projection against a
live-transaction store (Postgres or any future adapter at that end of the
axis) through `run_projection` rather than through the probe directly — or,
independently, a decision to publish `happenstance`'s runner ungated before
that case exists, the same "no operator has hit this gap yet" argument that
already carried the chunk-type and observation-seam question
(`kb-open-question-projection-runner-chunk-observation-001`) toward leaving
its 2A alone.

## A stale claim this atom corrects in passing

ADR-0063's own text says the typed layer's `unstable-projection` "no longer
forwards to the contract crate." At `86a410c`,
`crates/happenstance/Cargo.toml:142` reads
`unstable-projection = ["happenstance-core/unstable-projection",
"dep:futures-core"]` — it still forwards. That is the brief's statement
about the intended lane, not yet the state of the manifest; whoever executes
the Cargo-side half of ADR-0063 should close that gap in the same change
rather than let the ADR text and the manifest disagree past this wave.

## What this does not settle

Whether `ProjectionError`'s error arms need a new variant for a mid-`apply`
statement failure on a live batch; whether `Progressed` needs to report
anything different for a live consumer; and anything about the port itself,
which ADR-0063 already froze — this question is entirely on the typed
layer's side of that line.
