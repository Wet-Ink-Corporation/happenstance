---
id: kb-decision-0026
title: What a sync peer is — one relationship, one round trip, harmless re-delivery, and classified errors
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0026
reversibility: low
phase: 13
supersedes: null
superseded_by: null
summary: >-
  The number the original queue reserved for phase 13's "what is a peer". It makes one new call
  and records the rest. The new call is the owner's (wi-cbaec7, option A, 2026-10-14): every
  error a SyncPeer reports states its failure class, Transient, Split, Park or Stop, in a
  #[non_exhaustive] enum read through a bound on the associated Error, and a testkit rule drives
  every refusal a fixture can produce and checks its class. It is minted as SY-36, PROVISIONAL
  until that rule lands. Rejected: an opaque error, which is LiveStore's shipped failure (two of
  its four providers report every failure as UnknownError and "error out instead of
  reconverging"), and one concrete SyncError, which loses each adapter's own cause. What it
  records, and does not reopen: ingest is unconditional with compensation (SY-1 – SY-7); one
  peer per port and hub-ness is not a type (SY-8, SY-9); re-delivery is a no-op deduped on the
  store-assigned EventId, never a content hash (SY-11 – SY-13); one round trip per method, an
  owned resume token, and a bound on EventStore (SY-15 – SY-17), which is what lets a Postgres
  over one-shot HTTP be a peer. The published-surface half is ADR-0073. SY-18 is settled by
  #121's KV-capped peer, not here. The rejected design is LiveStore's single backend-arbitrated
  total order with client rebase: a rebased event lands after facts its append condition never
  saw. Two testing consequences from the same audit: device-local state goes through
  ProjectionStore, never local-only events in the log (SY-24), and a fault capability a fixture
  cannot provide is declined with a reason, never stubbed. Its envelope section records WF-1's
  renewal past 1.0, a separate happenstance-dcb-interop crate, with a named experiment: a bridge
  reading a UmaDB server's log over its published umadb.proto gRPC service.
depends_on:
  - kb-decision-wi-cbaec7
  - kb-decision-0073
  - kb-decision-0001
  - kb-decision-0003
  - kb-decision-0009
related:
  - kb-decision-wi-cbaec7
  - kb-decision-0073
  - kb-decision-0016
  - kb-decision-0028
  - kb-decision-0066
  - kb-open-question-dcb-no-published-format-001
  - kb-open-question-sync-message-set-undesigned-001
source_paths:
  - spec/SPECIFICATION.md
  - crates/happenstance-sync/src/peer.rs
  - crates/happenstance-sync/src/memory.rs
  - crates/happenstance-sync/tests/real_peer_shapes.rs
  - crates/happenstance-sync/tests/cursor_shape_probe.rs
  - runbook/ledgers.md
  - references/adr/0026-what-a-sync-peer-is.md
  - references/evaluation/livestore-prior-art-2026-10-11.md
last_reviewed: 2026-10-14
---

# What a sync peer is — one relationship, one round trip, harmless re-delivery, and classified errors

**Accepted by the owner on 2026-10-14.** The one new call is `wi-cbaec7`, option A; everything
else here is already in the specification and is recorded, not re-decided. The issue is #105, and
#119 (WF-1's envelope section) is folded in. It is `door:one-way`: the owner merges it. The long
form, with the evidence walked clause by clause, is
[`references/adr/0026-what-a-sync-peer-is.md`](../../references/adr/0026-what-a-sync-peer-is.md).

## Context

Phase 13 builds `happenstance-sync` against two real peers that disagree about every property a
Rust trait can express: a Durable Object over a socket, `!Send`, with errors behind an `Rc`; and a
Postgres over one-shot HTTP, `Send`, holding nothing between calls
(`crates/happenstance-sync/tests/real_peer_shapes.rs:7-13`). A port that the second cannot
implement is a port shaped like the first. Phase 2's sketch is the evidence: `SyncPeer` in two
flavours (`crates/happenstance-sync/src/peer.rs:81-158`), both stand-ins compiled against it, and
the cursor shape that compiled too, by buffering, and so had to be refused by a rule rather than
by the type checker (`crates/happenstance-sync/tests/cursor_shape_probe.rs:20-37`).

The specification has frozen most of the port since. One question it left open: the port's
`type Error: core::error::Error + 'static` (`crates/happenstance-sync/src/peer.rs:91`) is
opaque, so a runner cannot tell a dropped connection from a revoked credential from an event too
large for the peer. `SyncError` (`peer.rs:393-421`) covers only the runner's own failures.

## Decision

1. **A peer's error states its class (new; `wi-cbaec7`, option A).** The associated `Error` gains
   a bound — sketched as a `Classify` trait with `fn class(&self) -> FailureClass`; the exact
   identifiers are #243's — returning a `#[non_exhaustive]` enum:
   - `Transient`: retry, with jittered and capped backoff;
   - `Split`: the batch is over a limit; halve it and retry;
   - `Park`: this event can never land on this peer (VT-25's `ExceedsStoreLimit`); hold it and
     carry on with the rest;
   - `Stop`: authorisation or configuration; halt the edge and report it.

   The class is a function of the error alone. No class exists for a condition-level refusal,
   because SY-33 forbids that refusal (`spec/SPECIFICATION.md:8106-8129`). `SyncError`, the
   runner's own enum, states a class too: `BatchTooLarge` is `Split`, `BelowRetentionFloor` and
   `UnknownPeer` are `Stop`. It gains no variant here; the message set is ADR-0027's.
2. **It is minted as SY-36 `[PROVISIONAL]`** (`spec/SPECIFICATION.md:8150-8188`), with the rule
   `every_refusal_states_its_class` in `happenstance-sync-testkit` and a mutant that reports
   every failure as one class. The marker moves when the rule exists in the tree, not before;
   its 1.0 disposition is `freeze-by-13` (`runbook/ledgers.md`, the SY-36 row).
3. **Recorded, not reopened.** Ingest never refuses, and a conflict is compensated by an
   ordinary append (SY-1 – SY-7, `spec/SPECIFICATION.md:6963-7196`). `SyncPeer` describes one
   relationship and hub-ness is not a type (SY-8, SY-9, `:7195-7233`). Re-delivery is a no-op,
   deduped on the store-assigned `EventId` and never on a hash of the bytes (SY-11 – SY-13,
   `:7260-7357`). Every method completes in one round trip, resume state is an owned value the
   caller holds, and the port binds `EventStore` (SY-15 – SY-17, `:7417-7485`). Those three are
   what let a peer with no connection, transaction or cursor implement the port.
4. **The published-surface half is [ADR-0073](0073-the-foreign-identity-write-path-is-the-adapters.md).**
   `happenstance-core` grows no foreign-identity write path; the adapter's own row writer takes
   one, and VT-10 is frozen.
5. **SY-18 is not settled here.** #121's KV-capped peer settles whether a declared limit
   prevents the failure or relocates it. `Park` is what makes the failure it cannot prevent
   survivable.
6. **Two testing consequences of the LiveStore audit (#247).** Device-local state goes through
   `ProjectionStore`, as SY-31 already puts the watermark, never as local-only events in the log,
   because `[FROZEN]` SY-24 says a peer forwards every event it holds (`:7683`). And a fault
   capability a sync fixture cannot provide is declined with a stated reason, as the event-store
   testkit's are, never stubbed: LiveStore's offline test is a no-op for six of its seven
   providers.
7. **Envelope section: WF-1 renews past 1.0 (#119).** The format stays private
   (`spec/SPECIFICATION.md:2060`). Interop, if wanted, is a separate `happenstance-dcb-interop`
   crate with its own ADR, additive because WF-8 versions the envelope. The named experiment is a
   bridge that reads a UmaDB server's log over its published `umadb.proto` gRPC service and
   ingests it through `IngestStore`. The DCB reference still publishes no format; UmaDB's
   service is one implementation's client API, which is why it is the experiment rather than a
   standard to conform to.

## Consequences

- Every peer author implements one more trait, for every error variant. That is the cost the owner
  accepted for a runner that recovers without a human.
- A runner never retries an oversized event for ever, and never pages a human for a dropped
  connection.
- What the runner does for each class, and for a class added later behind `#[non_exhaustive]`, is
  #244's. The trait and the rule are #243's.
- SY-36 is the tenth `[PROVISIONAL]` `SY` clause, and the specification's census moves to 206.

## What reopens it

A refusal from a real peer whose right remedy depends on something the error does not carry, such
as its retry count or the batch that carried it (`wi-cbaec7`'s flip condition and SY-36's
falsifier). A refusal that needs a fifth remedy does not reopen it; a fifth class is additive.

## Alternatives that lost

- **An opaque error (B).** The runner can treat it only one way. LiveStore's Electric and S2
  providers do this and "error out instead of reconverging"
  (`livestorejs/livestore-contrib@c375b5ab`,
  `context/sync/electric/.delta/DELTA-001-no-rebase-signal.md`).
- **One concrete `SyncError` (C).** It drops each adapter's own cause, the `sqlx` or HTTP error,
  which every other port in the workspace keeps.
- **A runner-side table from error type to class.** One HTTP status type is `Stop` at 401, `Split`
  at 413 and `Transient` at 503. Only the adapter that produced it can say which.
- **One backend-arbitrated total order with client rebase** (LiveStore's sync core,
  `livestorejs/livestore@3cb74e62`). The backend refuses a non-fast-forward push, which SY-33
  forbids, and a client rebases its pending events, which SY-5 and SY-34 forbid. In DCB terms a
  rebased event lands after facts its append condition never saw, so the decision it was written
  under silently stops holding. LiveStore's own RFC 0002 names the cost: "Event rebasing gives us
  convergence without correctness."
