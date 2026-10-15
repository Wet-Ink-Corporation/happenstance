---
id: kb-decision-0098
title: An event claiming this store's own identity that it does not hold stops the ingest, and nothing is written
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0098
reversibility: high
phase: 13
supersedes: null
superseded_by: null
summary: >-
  Proposed on 2026-10-15 for #108, awaiting the owner. ADR-0073 left one ingest policy to phase 13:
  what a store does with a replicated event that claims the store's own StoreId at a position it
  does not hold. Until now the SQLite spike ingested it, and the local append that later reached
  the claimed position failed on UNIQUE (origin_store, origin_position) and wedged, because every
  retry was assigned the same position. This record refuses the whole ingest call instead, as
  IngestError::UnheldOwnIdentity { id }, class Stop, with nothing written. The insert decides it:
  a foreign row carrying the store's own identity that lands was not held. An own-identity row the
  store does hold is an ordinary skip. The remedy is VT-6's: an operator re-mints the store's
  identity, and the same batch then lands with the claim as a foreign event. It argues past
  ADR-0073's sentence that refusing would be a refusal on local state, which SY-1 forbids: the
  refusal is about the store's identity, its remedy is configuration (SY-33's permitted class), and
  it converges after the re-mint. It does not close VT-6's undetectable case, a restored store that
  has already re-issued the claimed identity to a different local event. New clause SY-38
  [PROVISIONAL]. Reversible while happenstance-sync is unpublished.
depends_on:
  - kb-decision-0026
  - kb-decision-0073
related:
  - kb-decision-0027
  - kb-decision-0028
  - kb-decision-wi-cbaec7
source_paths:
  - crates/happenstance-sync/src/ingest.rs
  - crates/happenstance-sync/src/ingest/error.rs
  - crates/happenstance-sqlite/src/event_store.rs
  - crates/happenstance-sqlite/src/ingest_spike.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-10-15
---

# An event claiming this store's own identity that it does not hold stops the ingest, and nothing is written

**Proposed on 2026-10-15 for #108, and awaiting the owner.** The session that wrote it built the
recommended option, because `happenstance-sync` is `publish = false` and every adapter impl of
`IngestStore` is still `#[cfg(test)]`, so the choice is free to change until the first publish. It
is not accepted until the owner accepts it. No long-form record exists; this atom is the record.

## Context

VT-6 says a `StoreId` names a store incarnation, and that a store restored from a backup or rewound
must re-mint it. A store that skips the re-mint can meet a replicated event that carries its own
`StoreId` at a position it does not hold: it minted that identity before the restore, and a peer
kept the event. [ADR-0073](0073-the-foreign-identity-write-path-is-the-adapters.md) left the policy
for that event to phase 13, and pinned what the SQLite spike did instead:

- the event was ingested, at the next local position, carrying the claimed identity;
- the local append whose assigned position later equalled the claim failed when its stamp met the
  claim on `UNIQUE (origin_store, origin_position)`;
- the rollback returned `AUTOINCREMENT`'s counter, so every retry was assigned the same position and
  failed the same way. Local appends were wedged until something moved the sequence or the
  identity.

ADR-0073 gave the reason it did not refuse: *"Refusing it would look like a refusal on local state,
which SY-1 forbids."*

## Options

| | Policy | Cost |
|---|---|---|
| P1 | Ingest it (the pin) | Wedges local appends later, at a distance from the cause, with a constraint error that names neither the claim nor the remedy. |
| **P2** | **Refuse the whole call as `UnheldOwnIdentity { id }`, class `Stop`, nothing written** | The edge halts until an operator re-mints. |
| P3 | Re-mint inside the ingest and take the claim as foreign | Needs a handle whose identity changes under `&self`, invalidates every other open handle, and Postgres and Neon cannot re-mint at all until #104. |
| P4 | Skip it silently | Drops a fact, which SY-4 forbids. |

## Decision

P2.

1. **The insert decides.** A foreign row that carries this store's own `StoreId` and **lands** was
   not held, because a held one meets the `UNIQUE` constraint and is skipped. No lookup runs ahead
   of the insert, so an echo of an event the store does hold, which SY-24 forwarding makes common,
   pays nothing extra and is a skip like any other re-delivery (VT-8's "decided by the write").
2. **The whole call is refused, and nothing in it is written.** SQLite drops its one transaction
   uncommitted, so the groups before the refusing one and every compensation roll back too. The
   claim may sit at any index of any group.
3. **The error is the port's.** `happenstance-sync` gains `IngestError<E>`, with four arms:
   `ExceedsStoreLimit`, `UnheldOwnIdentity`, `Busy` and `Store(E)`. `IngestStore::Error` is bound on
   `Classify` (SY-36), as ADR-0026 bound `SyncPeer::Error`, and `IngestError<E>: Classify` answers
   `Stop` for this arm. The arm is the port's, not the adapter's, so a runner and the ingest rules
   recognise it without knowing the adapter, and an adapter cannot spell the same fact as
   `Store(...)`.
4. **The remedy is the re-mint VT-6 already documents.** After it, the store's identity is new, the
   claim is a foreign event, and the same batch lands whole.

## Why this does not break SY-1

SY-1 says ingest never refuses on the content of the log, and SY-33 says a peer refuses only for
transport or authorisation reasons. The reason is that a refusal which depends on which events the
receiver holds can never succeed: the spoke cannot change the receiver's log, so retrying is an
infinite loop and the only exit is to discard a fact. ADR-0073's sentence reads this refusal as one
of those. This record argues it is not, on three grounds:

- **It is about the store's identity, not the events.** The question is whether this store is the
  incarnation that minted the identity, and the answer is no. It is a configuration fault in the
  receiver, the class SY-33 permits and the meaning `FailureClass::Stop` states.
- **Its remedy exists and is an operator's.** A condition-derived refusal has no remedy short of
  discarding a fact. This one has a documented one, the re-mint.
- **It converges.** After the re-mint, the same batch lands, unchanged, with nothing discarded
  (`a_claim_lands_as_foreign_after_the_store_is_reminted`). Without the remedy a retry fails the
  same way, which is what `Stop` tells a runner not to do.

P1, by contrast, also refuses, later and on local appends. It turns one ingest-time configuration
fault into an unbounded run of failed appends that carry no hint of the cause.

If the owner reads SY-1 more strictly than this, P1 stays, and only the error type changes.

## What it does not close

VT-6's undetectable case stays undetectable. A restored store that has **already** re-issued the
claimed identity to a different local event holds that identity, so the peer's original meets the
`UNIQUE` constraint and is skipped as a re-delivery. Two different events then carry one identity,
and nothing on this path can tell. P2 catches the first claim the store does **not** hold, which in
a forwarding topology usually arrives before the local appends reach the claimed position. It does
not catch this one, and neither did P1. The only defence is the re-mint VT-6 requires after a
restore.

## Evidence

In `crates/happenstance-sqlite/src/ingest_spike.rs`, all `#[cfg(test)]`, against the claim at
index 1 of the second of two groups:

- `an_unheld_own_identity_stops_the_ingest_and_writes_nothing`: the call fails as
  `UnheldOwnIdentity` naming the claim, class `Stop`, and neither group's events, the head nor the
  watermark changed.
- `local_appends_are_not_wedged_after_a_refused_claim`: five appends after the refusal land, and one
  takes the claimed position. This is the old wedge, gone.
- `a_claim_lands_as_foreign_after_the_store_is_reminted`: after `remint_identity` and a reopen, the
  same batch lands with `appended == 3`, and the claim is held under the identity that minted it.
- `a_compensation_whose_group_carries_an_unheld_own_identity_is_not_written`: the log is unchanged,
  compensation included.
- `an_echo_of_an_own_event_is_a_skip`: an own-identity event the store holds is a skip.

With detection disabled in `write_batch`, the first four fail. `ingest_reaches_a_foreign_store.rs`,
the stand-in adapter in `happenstance-sync`'s tests, makes the same check, so it stays a faithful
model of an adapter.

## Consequences

- SY-38 `[PROVISIONAL]` states the policy. Its rule is the SQLite unit test today, and the
  `happenstance-sync-testkit` rule `an_unheld_own_identity_stops_the_ingest` in #108's PR B.
- Every adapter that implements `IngestStore` owes the check, and it is decided by its own write:
  Neon and Postgres by the row a `RETURNING` reports, the Durable Object by its insert.
- ADR-0073's body is unchanged. It is accepted, and its sentence about SY-1 records why phase 17
  did not decide this. This record decides it.
- Reversible until `happenstance-sync` first publishes. After that, a peer that relies on the
  refusal, or on its absence, makes changing it a break.

## What would make this wrong

A deployment in which an unheld own identity is expected and benign, so that halting the edge costs
more than the wedge. One example would be a store that deliberately shares an incarnation with a
peer. VT-6 forbids that today, and a deployment that needs it reopens VT-6 first.
