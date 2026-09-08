# ES-11's falsifier fired, on the adapter its own marker named

**Date:** 2026-09-08
**Kind:** a `[PROVISIONAL]` clause falsified as predicted — reported, not settled
**Found by:** running the conformance suite against a live Neon endpoint at phase 10b
**Owner:** ES-11's, and it is owed an ADR before `happenstance-neon` can claim conformance

> **Settled on 2026-09-08 by ADR-0061**, whose brief is
> `2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md`. This file
> stays as the *finding*, unedited below this line, because the three options it
> lists and the reasons two of them lost are the argument the decision rests on.
> The choice was a fourth reading of option 1: the clause's async sufficiency
> sentence is **corrected** rather than its scope narrowed to exclude one-shot HTTP
> — a narrowing that removes a route to a conformance claim — together with option
> 2's stated limitation. **Two claims below did not survive the pass.** The CI cost
> is not *"red about 1 run in 40"* but **zero**, because `NEON_CONNECTION` is not a
> repository secret and every step of `live-neon` is gated on it. And *"104 of 105"*
> has a soft edge: `query_items_share_one_snapshot` is exposed to the same race by
> construction and has not yet lost it, which is a different statement from passing.

## What the marker said would happen

ES-11's `[PROVISIONAL]` marker, verbatim:

> axis: **transport**, whose far end is unbuilt. `MemoryEventStore` snapshots
> under the lock at call time and satisfies this for free; an adapter reaching
> its store over one-shot HTTP with no cursor can only self-paginate, and a
> self-paginating read is not a snapshot. **Falsified by the first one-shot-HTTP
> adapter that cannot meet this in one round trip — which is the outcome to
> expect**, and the reason to have settled it before the freeze rather than after.

`happenstance-neon` is that adapter. It exists now, it runs the suite, and the
predicted outcome is the one that arrived.

## What actually fails, and why it is not the failure the clause anticipated

`read_result_is_stable_under_concurrent_append` fails intermittently. Not because
the read self-paginates — it does not; the adapter buffers a whole result set in
one round trip, which is what ES-12 asks for and it holds by construction. It
fails because **a read and an append are two independent HTTPS requests to a
pooled proxy**, and nothing orders one backend's snapshot against another
backend's commit.

Measured over the conformance transport, 20 runs per configuration:

| transport | red runs |
|---|---|
| HTTP/1.1, default connection pool | **3 of 20** |
| HTTP/2, `pool_max_idle_per_host(1)` | **1 of 40** |

The adapter ships the second because a single multiplexed connection is the only
ordering primitive the transport offers. It reduces the race; it does not close
it.

## The sentence in the clause that is false for this shape

ES-11 states a sufficiency condition for asynchronous drivers, and it is right
about the driver it was written for and wrong about this one:

> A read spawned at its first poll and an append spawned afterwards **land in the
> same queue in that order**, so the snapshot precedes the append without either
> operation having completed when the poll returned.

That holds for `happenstance-postgres` because both operations enter one
`PgPool` — one queue, one ordering. It is false for a one-shot HTTP store by
construction: there is no queue, there are two requests, and the proxy may serve
them on different backends with different handshake costs.

**This is the distinction the clause has not got.** It already separates
synchronous drivers (the first poll *is* the sample) from asynchronous ones (the
sample is *committed to* at that poll). One-shot HTTP is a third shape, and the
clause's own paragraph about the second does not reach it.

## Read this against the last time ES-11 was escalated, because that one was wrong

`HANDOVER.md` records that ES-11 was escalated in error during the pre-publication
pass and retracted (`2e0a0ae`), and that commit `0341467`'s message asserting the
falsifier had fired is **wrong and is history, not guidance**. `ci.yml`'s
`live-postgres` job carries the same lesson in its own words: an earlier draft
asserted these two rules MUST fail *"on the conclusion that ES-11 could not be
satisfied by an asynchronous driver at all. That conclusion was wrong."*

**This is a different claim and it should be checked against that one before it is
accepted.** The retracted escalation said *no async driver can conform*, and was
refuted by an async driver conforming. This says *a driver with no shared queue
between its operations cannot conform*, which the refutation does not touch — the
thing that fixed `happenstance-postgres` was giving it one queue, and Neon cannot
have one.

## What is owed, and what was deliberately not done

**Not done, on purpose:** the specification was not amended, the rule was not
gated behind a capability, and no allowlist was added to CI. Amending a clause is
an ADR's work, and this clause is load-bearing — ES-11 and ES-12 *"reduce to
ES-10 plus a ceiling"*, so a careless widening reaches further than it looks.

**Owed:** an ADR that takes one of these.

1. **Amend ES-11** to name one-shot HTTP as a shape its sufficiency condition
   does not cover, and say what such an adapter must do instead — or that the
   obligation is unmeetable there and the clause's scope excludes it.
2. **Record that `happenstance-neon` does not satisfy ES-11**, leave the clause
   alone, and give the adapter a stated limitation. This is the option that costs
   the clause nothing and the adapter its conformance claim.
3. **Mint a capability** the fixture can decline, so the rule becomes a reported
   skip. This is the shape CF-18 already provides for *"this store cannot do
   that"* — and it is the option most likely to be wrong, because the rule is
   about a guarantee rather than about a fixture's ability to arm something.

## The cost of leaving it, which is real and is being paid

The `live-neon` CI job is kept **strict**: any failure is a failure, no named set
is tolerated. So the job will be red roughly **1 run in 40** for a reason that is
not a regression.

That is deliberate and it is the lesser of two bad options — `live-postgres`
already records that a gate tolerating a named set is one commit away from
tolerating the wrong one, and that it *had* tolerated a set that should never have
been there. But it is a cost with a half-life: a job that reddens at random
teaches people to re-run rather than to read, which is the same defect as a green
tick nobody re-reads, arriving from the other side.

**So the ADR is not optional and it is not slow-burning.** Until it lands, the
honest statement of this adapter's conformance is *104 of 105 rules, with one
open clause question* — not 105.
