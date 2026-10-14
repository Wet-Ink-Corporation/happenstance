---
id: "kb-decision-wi-cbaec7"
title: "ADR-0026 (#105): how should SyncPeer::Error tell the runner what to do with a failure (retry, split, park, stop): A"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"ADR-0026 (#105): how should SyncPeer::Error tell the runner what to do with a failure (retry, split, park, stop)?\", facing a runner that cannot tell a dropped connection from a revoked credential from an event too big for the peer, we decided for A: the port's error states its class, accepting one more trait obligation on every peer author. Neglected: B, stay opaque; C, one concrete error type."
depends_on: []
related:
  - kb-decision-0026
  - kb-decision-0009
source_paths:
  - crates/happenstance-sync/src/peer.rs
  - crates/happenstance-sync/src/memory.rs
  - crates/happenstance-sync/tests/real_peer_shapes.rs
  - spec/SPECIFICATION.md
  - references/adr/0026-what-a-sync-peer-is.md
last_reviewed: "2026-10-14"
reversibility: low
phase: 13
supersedes: []
superseded_by: null
weighin_item: "wi-cbaec7"
question: "ADR-0026 (#105): how should SyncPeer::Error tell the runner what to do with a failure (retry, split, park, stop)?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-14T01:09:50Z"
tree_hash: "2f25eccf8841e601a8ba820350af089070f5f579"
recommended: "A"
flip_condition: "a real peer's refusal whose remedy depends on context the error does not carry"
---

# ADR-0026 (#105): how should SyncPeer::Error tell the runner what to do with a failure (retry, split, park, stop): A

## Context and problem statement

ADR-0026 (#105): how should SyncPeer::Error tell the runner what to do with a failure (retry, split, park, stop)?

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-cbaec7`. Anchor: `crates/happenstance-sync/src/peer.rs:91`.

`SyncPeer::Error` is an associated type bounded only by `core::error::Error + 'static`, so a
runner holding one cannot tell which remedy it calls for. LiveStore audit finding L-16 (#243):
two of LiveStore's four sync providers report every failure as `UnknownError` and "error out
instead of reconverging".

## Considered options

### A · Classify (chosen)

The port's error gains `class()`, returning a `#[non_exhaustive]` `FailureClass`: `Transient`
(retry with backoff), `Split` (batch too large: halve and retry), `Park` (this event can never
land on this peer, VT-25's `ExceedsStoreLimit`: hold it, continue with the rest), `Stop`
(authorisation or configuration: halt the edge and report). A testkit rule drives every refusal
a fixture can produce and checks its class.

- Holds if every refusal's remedy is a function of the error alone.
- If wrong: an adapter must guess a class for a refusal whose remedy depends on context.

### B · Stay opaque

The runner treats every error as transient and retries.

- If wrong: a permanently oversized event retries for ever and blocks the edge. This is the
  LiveStore failure.

### C · One concrete error type

Drop the associated type for a single `SyncError` shared by every peer.

- If wrong: each adapter's own cause (its `sqlx` or HTTP error) is lost, which every other port
  in this workspace keeps.

## Decision outcome

Chosen option: **A**, the recommended option.

Decider's note: Ryan 2026-10-14: 'Classify'. SyncPeer::Error gains a class() -> #[non_exhaustive] FailureClass {Transient, Split, Park, Stop}; testkit rule checks every refusal's class.

Recorded in [ADR-0026](0026-what-a-sync-peer-is.md), which mints SY-36 `[PROVISIONAL]` for it.
The trait and the rule are #243's to build; what the runner does for each class is #244's.

## Provenance

- Decided 2026-10-14T01:09:50Z by human:ryan (user-directed), via chat.
- Raised in session `2fe67e8e-8afc-5d1f-abc9-f48820905de5`.
- Verbatim: "ADR-0026 (#105): how should SyncPeer::Error tell the runner what to do with a failure (retry, split, park, stop)?"
