# ES-11's falsifier has fired, on an axis its marker did not name, and it gates `0.2.0`

Record: **ES-11-FIRED**. Staged for `/redkiln:kb-ingest`. Escalated by the
repository owner at the `0.2.0` release pass, 2026-09-07.

**This is not a scheduled falsifier. It is a fired one, on a clause published at
`0.2.0`, and the difference is the whole reason this brief exists.**

---

## What ES-11 says

> One `read` call MUST be evaluated against a single consistent state of the
> store, **fixed no later than the first poll of the returned stream.** Events
> appended after that instant MUST NOT be yielded.

## What its marker predicted

> **[PROVISIONAL — axis: transport, whose far end is unbuilt.** `MemoryEventStore`
> snapshots under the lock at call time and satisfies this for free; an adapter
> reaching its store over one-shot HTTP with no cursor can only self-paginate,
> and a self-paginating read is not a snapshot. **Falsified by the first
> one-shot-HTTP adapter that cannot meet this in one round trip** — which is the
> outcome to expect, and the reason to have settled it before the freeze rather
> than after.]

The marker named **transport**, named the instrument — `happenstance-neon` — and
called the falsification *"the outcome to expect"*.

## What actually falsified it

**`happenstance-postgres`**, which has a connection, an interactive transaction
and a cursor. Not the transport axis at all. From the lane's own commit:

> ES-11 wants a read's state fixed no later than the first poll; **an async
> driver's first poll can only START the round trip that takes the snapshot.**
> `happenstance-sqlite` passes because rusqlite is synchronous. Spawning onto the
> runtime at the first poll narrows the window and does not close it (5 failures
> in 5 runs); opening in `read` would close it and break ADR-0011 and AC-012.
> ES-11 is `[PROVISIONAL]` and names its own falsifier — it expected the
> transport axis, and what arrived is **the driver being asynchronous at all,
> which is every adapter here but one.**

Two rules fail deterministically against a live pinned Postgres. The lane does
not skip them: its job asserts both failures are **present**, tolerates a
recorded frontier-sensitive set, and fails on anything else.

## Why this is bigger than the marker anticipated

The marker scoped the exposure to one unbuilt adapter on one axis. The real
exposure is **every asynchronous driver**, which is every adapter in this
workspace except `happenstance-sqlite` — and SQLite is the exception only because
`rusqlite` is synchronous, which is a property of a C library binding rather than
a design choice this project made.

A clause whose falsifier fires on the *predicted* instrument is a clause working
as designed. A clause whose falsifier fires on an instrument it did not name,
through a mechanism it did not consider, is a clause whose **premise** was wrong,
not merely its schedule.

## Why it gates the release

`RUNBOOK.md`'s phase 12 exit criteria:

> Every `[PROVISIONAL]` clause published at 0.1 either has its falsifier
> scheduled in a later phase of this file, or is behind an unstable feature.

ES-11 satisfies neither reading honestly. Its falsifier is not *scheduled* — it
has **fired**. And it is not behind an unstable feature: it is an `ES` clause on
`EventStore`, the port that ships stable at `0.2.0`, where §1.3 makes a `[FROZEN]`
`ES` clause semver-binding and a provisional one a promise about where the
contract is going.

Publishing `0.2.0` with ES-11 as written means shipping a MUST that two of the
three real adapters cannot meet, and saying so nowhere a consumer reads.

## What is *not* claimed here

- **Not that the adapters are wrong.** The lane's analysis is that the two
  available repairs are worse: spawning at the first poll narrows the window
  without closing it, measured at 5 failures in 5 runs; opening the transaction
  inside `read` would close it and break ADR-0011 and AC-012.
- **Not that ES-11 should be deleted.** The snapshot property is what makes a
  read safe to fold a decision model from. What is in question is *where the
  instant is fixed*, not whether one exists.
- **Not that this brief knows the answer.** It does not. The shape of a repair —
  amend the clause to permit the instant to be fixed at the first *statement*
  rather than the first *poll*; narrow the MUST to synchronous drivers; or accept
  a documented divergence per adapter — is an ADR's work, and this brief is an
  escalation rather than a proposal.

## The options, named without recommendation

- **A — amend ES-11 before `0.2.0`.** Move the instant from "first poll" to
  something an async driver can honour. Touches a clause on a published surface;
  cheapest while nothing is published.
- **B — narrow the clause's scope.** ES-11 binds synchronous drivers as written
  and asynchronous ones under a stated weaker guarantee. Honest, and it splits a
  MUST in two.
- **C — publish with ES-11 unchanged and the divergence documented per adapter.**
  Cheapest today, and it ships a clause the specification's own instruments
  contradict.
- **D — hold `0.2.0` until the clause is settled.** The conservative reading, and
  the one the F2-5 episode of the previous day argues both for and against.

## Related

- [[f2-5-holds-the-release-for-phase-10]] — the other clause-level finding from
  the same lane, discharged rather than escalated.
- [[es-42-marker-earned-off-at-0-2-0]] — the marker that named a deadline and was
  settled before it.
- `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` — the
  standing question about where the visibility invariant is scoped.
- ADR-0011, ADR-0024 (arm C), AC-012 — the constraints any repair has to survive.
