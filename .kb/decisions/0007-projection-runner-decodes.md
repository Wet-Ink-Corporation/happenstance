---
id: kb-decision-0007
title: The projection runner decodes, and therefore splits across the seam
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0007
reversibility: medium
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Partly supersedes ADR-0006's runner allocation while leaving its naming decision untouched and
  standing. The runner splits at the decode boundary: happenstance-core holds the checkpoint pump,
  which reads from a checkpoint, chunks, begins a batch, invokes a callback per SequencedEvent and
  commits with the last position applied, and never decodes; happenstance holds the Projection
  trait and the runner an application actually uses, with decoded events, the projection's Query
  and the store's Batch. ADR-0006's discriminator — encoding, not orchestration — survives intact;
  what fails is its claim that the whole runner never decodes, asserted before any runner existed.
  Three shape decisions travel with it: a projection nominates events with Query, the same type a
  decision model uses, so there is no second filtering vocabulary; Projection::Store is an
  associated type, so a projection spanning two stores is unrepresentable; and checkpoints stay
  per store and projection id, so callers reading two stores must tolerate the skew. Falsifier: if
  the core pump has acquired no caller but the typed one when phase 7 exits, collapse it upward and
  supersede this decision.
depends_on:
  - kb-decision-0006
related: []
source_paths:
  - .kb/_intake/0007-projection-runner-decodes.md
  - references/adr/0007-projection-runner-decodes.md
  - crates/happenstance-core/src/projection.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# The projection runner decodes, and therefore splits across the seam

## Context

ADR-0006 allocated the bare crate name to the typed layer and, in the same decision, moved the
projection runner into the contract crate on the grounds that the discriminator for what belongs
in the typed layer is encoding, not orchestration — "the projection runner pumps an `EventStore`
into a `ProjectionStore` and never decodes a payload." The discriminator held up. The claim about
the runner did not: it was asserted before any runner existed, and testing it against an ordinary
consumer — an application projecting SQLite events into both a SQLite and a LadybugDB read model
— shows a runner that never decodes hands back opaque `Bytes` and leaves every application to
hand-write the same decode, per view. That is the easy half of a two-part job sold as the whole
job.

## Decision

Split the runner at the decode boundary. `happenstance-core` keeps the checkpoint pump: it reads
from a checkpoint, chunks, `begin`s a batch, invokes a callback per `SequencedEvent`, and commits
with the last position applied — never decoding. `happenstance` gets the `Projection` trait and
the runner an application actually calls, working in decoded events, the projection's `Query`, and
the store's `Batch`. ADR-0006's discriminator survives untouched; only the claim that the runner
falls wholly on the core side of it is corrected. The transactional invariant and the decode are
different jobs, and the invariant belongs beside the port that defines it.

Three shape decisions ride with the split. A projection nominates its events with `Query` — the
same type a decision model uses — so there is no second filtering vocabulary, and a projection's
subscription is checkable against semantics the conformance suite already pins. `Projection::Store`
is an associated type, making a projection that spans two stores unrepresentable at compile time
rather than merely undocumented — correct, because there is no cross-store transaction and one
that appeared to work would misrepresent the invariant `ProjectionStore` exists to defend.
Checkpoints stay per `(store, ProjectionId)`, so two stores projecting the same events sit at
different positions at any given moment; that skew is inherent, and callers reading both must
tolerate it.

## Consequences and alternatives rejected

The convenience lands where the consumer meets it, and `happenstance-core` keeps the checkpoint
invariant beside the port that states it. The cost is two runners to document, and a real risk
that the core pump acquires no caller but the typed one — its own falsifier, stated above.
Rejected: leaving the whole runner in `happenstance-core` and letting the application decode
inside `apply` (cheapest, but every application then hand-rolls the identical wrapper); moving the
whole runner into `happenstance` (simpler, but it puts the checkpoint invariant in a crate that
sits above the port and can be swapped out — this is the fallback if the falsifier fires); and a
projection store that decodes (rejected on ADR-0003's reasoning: a store that parses payloads
carries domain knowledge, and the adapter population must not).

ADR-0006's body stays verbatim per this corpus's supersession rule; only its runner allocation is
partly superseded here, and its naming decision — the bare name to the typed layer, the contract
renamed to `happenstance-core` — stands on its own reasoning, untouched.
