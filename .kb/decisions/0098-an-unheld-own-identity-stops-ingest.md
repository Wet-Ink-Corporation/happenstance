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
  did not hold. Until now the SQLite spike ingested it, and the local append that later reached
  the claimed position failed on UNIQUE (origin_store, origin_position) on every retry. This record
  refuses the whole ingest call instead, as IngestError::UnheldOwnIdentity, class Stop, with
  nothing written, and the remedy is VT-6's re-mint. It AMENDS two frozen clauses with a narrow
  identity-integrity exception: SY-1, which says an ingesting peer "MUST append every event it
  receives that it does not already hold", and SY-33, which forbids "any refusal whose truth value
  depends on which events the receiver holds". The exception is minted as SY-38 [PROVISIONAL]; the
  frozen texts are left unedited, and this record lists the edits its acceptance makes to them. It
  does not close VT-6's undetectable case. Reversible while happenstance-sync is unpublished.
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

**This record amends two `[FROZEN]` clauses, SY-1 and SY-33.** It does not read them narrowly, and
it does not claim that the refusal it adds falls outside their words. It does. Acceptance is the
owner's decision to carve a narrow exception out of both, and the edits that acceptance makes to
their text are listed below under "What acceptance changes".

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
  failed the same way. Local appends were wedged from that position on, until something moved the
  sequence or the identity.

ADR-0073 gave the reason it did not refuse: *"Refusing it would look like a refusal on local state,
which SY-1 forbids."* That reading is correct, and this record does not dispute it.

## The clauses this amends, as they stand

SY-1 (`spec/SPECIFICATION.md`, §5.1), `[FROZEN]`:

> **SY-1. Ingest MUST NOT refuse a replicated event for any reason that is a function of the
> receiving store's state.**
>
> An ingesting peer MUST append every event it receives that it does not already hold. In
> particular it MUST NOT evaluate an `AppendCondition` — its own or the origin's — as a
> precondition on the append.

SY-33 (§5.10), `[FROZEN]`:

> **SY-33. A peer MAY refuse a push for transport-level or authorisation-level reasons. It MUST
> NOT refuse for a condition-level reason.**
>
> Refusals permitted: unreachable, unauthenticated, unauthorised for the tag space pushed,
> malformed envelope, quota. Refusals forbidden: any refusal whose truth value depends on which
> events the receiver holds.

The refusal this record adds is a function of the receiving store's state: whether it holds an
event under its own identity. It refuses an event the store does not already hold. And its truth
value depends on which events the receiver holds. So it breaks SY-1's title and its first MUST, and
SY-33's forbidden list, as written.

## Options

| | Policy | Cost |
|---|---|---|
| P1 | Ingest it (the pin) | Wedges local appends from the claimed position on, far from the cause, with a constraint error that names neither the claim nor the remedy. Keeps SY-1 and SY-33 as written. |
| **P2** | **Refuse the whole call as `UnheldOwnIdentity`, class `Stop`, nothing written** | Amends SY-1 and SY-33. The costs are listed under "Costs" below. |
| P3 | Re-mint inside the ingest and take the claim as foreign | Needs a handle whose identity changes under `&self`, invalidates every other open handle, and Postgres and Neon cannot re-mint at all until #104. |
| P4 | Skip it silently | Drops a fact, which SY-4 forbids. |

## Decision

P2, as a narrow exception to SY-1 and SY-33.

1. **What is refused.** A replicated event whose `EventId` carries the receiving store's own
   `StoreId`, where the store did not hold that identity when the ingest call began. Nothing else:
   an event carrying any other store's identity is still appended or skipped exactly as SY-1 says.
2. **The write decides it**, with no lookup ahead of it (VT-8). In SQLite there are two ways for
   the insert to show it:
   - the row **lands**, so the store did not hold it;
   - the row is **skipped** under `UNIQUE`, but against a row this same call wrote. A compensation
     earlier in the call is stamped with the store's own identity, and a claim on its position
     reads as a re-delivery. The head is read once under the write lock when the call begins, and
     an own-identity skip above it is refused too. Without this, the call would commit two events
     under one `EventId`.
   An own-identity event the store held before the call is a skip, as before.
3. **The whole call is refused, and nothing in it is written.** SQLite drops its one transaction
   uncommitted, so the groups before the refusing one and every compensation roll back too. The
   claim may sit at any index of any group.
4. **The error is the port's.** `happenstance-sync` gains `IngestError<E>`, with four arms:
   `ExceedsStoreLimit`, `UnheldOwnIdentity`, `Busy` and `Store(E)`. `IngestStore::Error` is bound on
   `Classify` (SY-36), as ADR-0026 bound `SyncPeer::Error`, and `IngestError<E>: Classify` answers
   `Stop` for this arm. The arm is the port's, so a runner and the ingest rules recognise it without
   knowing the adapter, and an adapter cannot spell the same fact as `Store(...)`.
5. **The remedy is the re-mint VT-6 already documents.** After it, the store's identity is new, the
   claim is a foreign event, and the same batch lands whole.

## Why the exception is narrow enough to take

SY-1 and SY-33 exist because a refusal on the receiver's log contents can never succeed: the
sender cannot change the receiver's log, so a retry loops for ever, and the only exit is to discard
a fact. This refusal is a function of the receiver's state, but a different part of it:

- **It is about the receiver's identity, not the events' content.** The question is whether this
  store is the incarnation that minted the identity, and the answer is no. That is a configuration
  fault in the receiver, the class SY-33's permitted list is made of, and the meaning
  `FailureClass::Stop` states.
- **It has a remedy, and the remedy is an operator's.** A condition-derived refusal has none short
  of discarding a fact. This one has the re-mint.
- **It converges.** After the re-mint, the same batch lands unchanged, with nothing discarded
  (`a_claim_lands_as_foreign_after_the_store_is_reminted`).

## Costs

- **Postgres and Neon have no re-mint until #104.** When they implement `IngestStore`, an edge into
  either that meets an unheld own identity stops, and the documented remedy does not exist there
  yet. Until #104 lands, the only recovery on those stores is manual.
- **A buggy or forging peer can halt an edge.** A peer that sends an event carrying the receiver's
  `StoreId`, through a bug or deliberately, stops that edge, and re-minting is the wrong remedy: the
  receiver was never restored, and a re-mint splits its history into two origins for nothing. The
  operator has to tell a restored store from a misbehaving peer before acting, and the error does
  not say which it is.
- **`Stop` halts the whole edge, for every origin it carries.** P1's wedge halts this store's local
  appends from the claimed position on and leaves replication flowing. P2 keeps local appends
  working and halts replication on the edge, so events from every other origin on it wait too, until
  someone acts.

If the owner weighs these above the wedge, P1 is the fallback: restore the pin test and keep only
the error type. SY-38 and the amendments below then go, and SY-1 and SY-33 stand as written.

## What acceptance changes

Acceptance is the owner's act, and it makes three edits this record does not make:

- **SY-1**, after its second paragraph: *"SY-38 is the single exception: an event that claims the
  receiving store's own `StoreId`, where the store did not hold that identity when the ingest
  began, is refused with its whole call."*
- **SY-33**, after "Refusals forbidden: …": *"The one refusal that depends on which events the
  receiver holds and is permitted is SY-38's, of an event claiming the receiver's own identity that
  it does not hold; it is an identity-integrity refusal, not a condition-level one."*
- **SY-33's rule**, `refusal_is_never_condition_derived`, when it is written in
  `happenstance-sync-testkit`: it exempts `IngestError::UnheldOwnIdentity` from "none of them
  varies with the receiver's log contents", and SY-38's own rule covers that refusal.

SY-38 `[PROVISIONAL]` states the exception now, and says it carves out of SY-1 and SY-33. Their
`[FROZEN]` texts stay unedited until acceptance.

## What it does not close

VT-6's undetectable case stays undetectable. A restored store that has **already** re-issued the
claimed identity to a different local event, in an earlier call, held it when the ingest began, so
the peer's original is skipped as a re-delivery. Two different events then carry one identity, and
nothing on this path can tell. P2 catches every claim the store did not hold when the call began,
including one hidden by the call's own compensation. It does not catch this one, and neither did
P1. The only defence is the re-mint VT-6 requires after a restore.

## Evidence

In `crates/happenstance-sqlite/src/ingest_spike.rs`, all `#[cfg(test)]`:

- `an_unheld_own_identity_stops_the_ingest_and_writes_nothing`: the claim at index 1 of the second
  group; the call fails as `UnheldOwnIdentity` naming it, class `Stop`, and neither group's events,
  the head nor the watermark changed.
- `a_claim_the_calls_own_compensation_would_hide_is_refused`: group 1's compensation takes the
  claimed position, and the call is still refused with the log unchanged. It was red before the
  head check: `Ok(Ingested { appended: 2, skipped: 1, compensated: 1 })`, two events under one id.
- `local_appends_are_not_wedged_after_a_refused_claim`: five appends after the refusal land, and one
  takes the claimed position.
- `a_claim_lands_as_foreign_after_the_store_is_reminted`: after `remint_identity` and a reopen, the
  same batch lands with `appended == 3`.
- `a_compensation_whose_group_carries_an_unheld_own_identity_is_not_written`: the log is unchanged,
  compensation included.
- `an_echo_of_an_own_event_is_a_skip`: an own-identity event the store held is a skip.

With detection of the landed claim disabled in `write_batch`, the four landed-claim tests fail. The
stand-in adapter in `happenstance-sync`'s `tests/ingest_reaches_a_foreign_store.rs` makes both
checks, so it stays a faithful model of an adapter.

## Consequences

- SY-38 `[PROVISIONAL]` states the exception. Its rule is the SQLite unit test today, and the
  `happenstance-sync-testkit` rule `an_unheld_own_identity_stops_the_ingest` in #108's PR B, which
  must include the compensation-hidden arrangement.
- Every adapter that implements `IngestStore` owes both checks, decided by its own write: Neon and
  Postgres by the rows a `RETURNING` reports and the head at the statement's start, the Durable
  Object by its insert.
- ADR-0073's body is unchanged. It is accepted, and its sentence about SY-1 records, correctly, why
  phase 17 did not decide this.
- Reversible until `happenstance-sync` first publishes. After that, a peer that relies on the
  refusal, or on its absence, makes changing it a break.

## What would make this wrong

A deployment in which an unheld own identity is expected and benign, so that halting the edge costs
more than the wedge. One example would be a store that deliberately shares an incarnation with a
peer. VT-6 forbids that today, and a deployment that needs it reopens VT-6 first.
