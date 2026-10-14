---
id: kb-decision-0027
title: Replication is whole-log at 1.0, and the merge rule, bulk ingest and both topologies are ratified
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0027
reversibility: low
phase: 13
supersedes: null
superseded_by: null
summary: >-
  The owner's two calls of 2026-10-14 for #106. First (wi-53c544, option A), replication is
  whole-log at 1.0. A confidentiality or size slice is its own store, which is LiveStore's
  production answer (many small stores, no filtered sync). SY-27 is settled as whole-log and
  SY-28's round-trip rule asserts whole-log equality over EventId sets, never positions; both
  move from DEFERRED to PROVISIONAL, falsified by a real pre-1.0 deployment that needs filtered
  spokes and cannot use one store per slice. SY-29 leaves the 1.0 path because no port method
  takes a peer-supplied Query, and is renewed past 1.0 against the first scoped-replication
  proposal, which comes with its own ADR as an additive opt-in. #122, the filtered-subset store,
  leaves the 1.0 path, CF-27's suffix store stays phase 14's, and the ledger's sequencing hazard
  is gone. Scoped at 1.0 and running SY-27's experiment first are rejected. Second (wi-0fbcac,
  option A), SY-7, SY-10, SY-14 and SY-23 are ratified with three edits: SY-10's rationale is
  adjudication asymmetry rather than Kestrel Cold Chain's confidentiality slice; SY-14's bound is
  one ingest round trip per pulled batch, however many of its events are already held; SY-23
  gains the caveat that EventId order is neither fair nor temporal and never decides a conflict.
  LiveStore's total order plus client rebase is considered and rejected. No marker moves to
  FROZEN with this record; each moves when phase 13 lands its rule or measurement.
depends_on:
  - kb-decision-0014
  - kb-decision-0066
  - kb-decision-0073
  - kb-decision-wi-53c544
  - kb-decision-wi-0fbcac
related:
  - kb-decision-0028
  - kb-decision-wi-53c544
  - kb-decision-wi-0fbcac
source_paths:
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
  - runbook/phases/13-sync.md
  - crates/happenstance-core/src/identity.rs
  - crates/happenstance-sync/src/ingest.rs
  - crates/happenstance-sqlite/src/ingest_spike.rs
  - references/scenarios/README.md
  - references/adr/0027-whole-log-replication-and-the-merge-rule.md
last_reviewed: 2026-10-14
---

# Replication is whole-log at 1.0, and the merge rule, bulk ingest and both topologies are ratified

**Accepted by the owner on 2026-10-14**, in two calls: `wi-53c544` (*"Whole-log"*) and
`wi-0fbcac` (*"freeze all 4 with edits"*). The long form, with the options, the evidence and
the marker table, is
[`references/adr/0027-whole-log-replication-and-the-merge-rule.md`](../../references/adr/0027-whole-log-replication-and-the-merge-rule.md).

## Context

ADR-0027 was reserved for phase 13's second question: how two logs reconcile. SY-1 – SY-7
already fixed *ingest is unconditional, with compensation*, and SY-1 – SY-6 are `[FROZEN]`.
This record writes them down as settled and decides what was still open: whether replication
is whole-log or scoped (SY-27 – SY-29), the adjudicator (SY-7), the topologies (SY-10), bulk
ingest (SY-14) and the deterministic order (SY-23).

## Decision

1. **Replication is whole-log at 1.0.** A peer offers its counterpart every event it holds, and
   the port carries no peer-supplied filter. A slice that must not be replicated is its own
   store. SY-27 now says so, and SY-28's round-trip rule asserts whole-log equality: the same
   set of `EventId`s, byte-identical, never compared by position. Both are `[PROVISIONAL]`.
2. **SY-29 leaves the 1.0 path.** No 1.0 port method takes a peer-supplied `Query`, so its MUST
   binds nothing at 1.0. It is renewed past 1.0 against the first scoped-replication proposal,
   which must come with its own ADR, as an additive opt-in that leaves whole-log peers alone.
   A pushed guard still travels as evidence (SY-6) and ingest never evaluates it.
3. **#122 leaves the 1.0 path.** The filtered-subset store was the instrument for a scoped
   answer. CF-27's suffix store stays phase 14's, and the sequencing hazard the ledger recorded
   between phases 13 and 14 no longer exists.
4. **SY-7 is ratified as written.** One adjudicator per fact family, chosen by runner
   configuration. Duplicate compensations cannot be made to converge: an `EventId` is the
   writing store and its position (`crates/happenstance-core/src/identity.rs:96`, VT-5, SY-12),
   so the same compensation written on two peers gets two ids, and SY-13 forbids the
   content-hash identity that would dedupe them.
5. **SY-10 is ratified, with adjudication as its reason.** The hub sees every spoke and
   adjudicates, *first to the hub, not first to decide* (`references/scenarios/README.md:436`);
   a spoke sees only itself. Kestrel Cold Chain's confidentiality slice no longer argues for the
   topology, because under decision 1 that slice is a separate store.
6. **SY-14's bound is one ingest round trip per pulled batch, however many of its events are
   already held.** The old wording, *"bounded independently of the batch size"*, names no bound
   a rule can count; a pull is already page-bounded. The mechanism is built: the unique
   `(origin_store, origin_position)` index in all four adapters, the SQLite spike's
   `ON CONFLICT … DO NOTHING` (`crates/happenstance-sqlite/src/ingest_spike.rs:12`), and
   `IngestStore::ingest` as one call per pulled batch
   (`crates/happenstance-sync/src/ingest.rs:157`). The marker stays `[DEFERRED]` until the Neon
   measurement passes: Kestrel Rotor's 1,840 events over one-shot HTTP in 34 minutes.
7. **SY-23 is ratified with a caveat.** `EventId` order is deterministic but neither fair nor
   temporal: the lower-sorting store's events always come first. It must never decide who wins
   a conflict; arrival at the adjudicator does (SY-7). A domain that wants *first in real time*
   carries a writer's clock in a tag and accepts skew. `StoreId` and `EventId` already derive
   `Ord`; an HLC would be a breaking new field, and VT-9 forbids `RecordedAt` as an order.

## Markers

No clause moves to `[FROZEN]` here. The owner's *"freeze"* is carried out as ratification:
each marker moves in the change that lands its rule or measurement, because in this repository
a marker moves on evidence in the tree (ADR-0066's *"Thirteen of fourteen proposed freezes were
refuted"*, and phase 13's *"the decorative kind of freeze"*). SY-27 and SY-28 move from
`[DEFERRED]` to `[PROVISIONAL]`. SY-29 stays `[PROVISIONAL]` with a renewed falsifier and a
`renew-past-1.0` disposition. SY-7, SY-10 and SY-23 stay `[PROVISIONAL]`, and SY-14 stays
`[DEFERRED]`, with their text edited as above.

## Consequences

- A fact that belongs to two slices is written to two stores, and a hub's adjudicator cannot
  see across slices. That is the price of decision 1, and the deployment that wants slices
  pays it.
- While an adjudicator is unreachable, a conflict stays unresolved on every other peer. If it is
  lost for good, an operator moves the role in configuration.
- `SyncPeer::Resume` stays an opaque associated type, which keeps a scoped opt-in additive.
- Phase 13 drops its riskiest item. #140 freezes SY-27 and SY-28 on
  `round_trip_preserves_the_whole_log` and re-dispositions SY-29; #122 can close as not planned
  for 1.0.

## Alternatives that lost

- **Scoped at 1.0** (`wi-53c544` B). The biggest piece of phase 13: a version-vector resume
  against a filtered hub, a `Query` on the port, SY-29's cost policy and #122.
- **Run SY-27's experiment first** (`wi-53c544` C). The same build as B, before knowing it is
  needed.
- **Ratify verbatim** (`wi-0fbcac` B). SY-14 would stay untestable as worded.
- **Total order plus client rebase**, LiveStore's design: a backend refuses a non-fast-forward
  push and clients rebase pending events. Its own RFC 0002 calls the result *"convergence
  without correctness"*; in DCB terms a rebase moves an event past facts its condition never
  saw, and refusing by receiver state is what SY-1 and SY-33 forbid (audit L-01, L-02; #247).

## What reopens it

A real deployment before 1.0 that needs filtered spokes and cannot use one store per slice
reopens decision 1, and the answer is a scoped opt-in with its own ADR. Two peers that must
both adjudicate one family with convergent compensations reopen SY-7. A Neon measurement that
fails changes SY-14's ingest path or records a deployment limit; it does not restore a
per-event condition.
