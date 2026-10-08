---
id: kb-open-question-one-shot-http-es-11-001
title: Whether any one-shot-HTTP adapter can satisfy ES-11, now that the first one cannot
kind: open_question
status: superseded
superseded_by: kb-decision-0087
authority_tier: note
summary: >-
  ADR-0061 amended ES-11's asynchronous-driver sufficiency condition to
  require that a read be spawned at the first poll AND ordered against a
  later append by something the store itself honours, and recorded that
  happenstance-neon does not satisfy ES-11 as a stated limitation. ES-11
  keeps [PROVISIONAL], with its marker rewritten to record a fired falsifier
  rather than predict one, and what it now holds open is this: whether a
  conformant one-shot-HTTP shape exists at all. The transport offers exactly
  one ordering primitive — HTTP/2 over a single multiplexed connection, which
  happenstance-neon ships and which narrows the race without closing it —
  and nothing else in a pooled-proxy path orders one backend's snapshot
  against another backend's commit. So the question is not whether this
  adapter can be fixed but whether the obligation is meetable by the shape
  at all, and what an adapter would have to supply if it were: a
  store-honoured ordering primitive the endpoint does not currently expose,
  an endpoint guarantee this ADR did not find, or an admission that the
  clause's scope has a hole its own text forbids widening (ES-11 and ES-12
  reduce to ES-10 plus a ceiling, so a widening reaches append-condition
  correctness). Nothing forces an answer today, because NEON_CONNECTION is
  not a repository secret, every live-neon step is gated on it, and the job
  prints its own proves-NOTHING notice and stops — which is also why the
  decision to keep that job strict should be re-read the day someone adds
  the secret rather than before it.
  Resolved 2026-10-08 by kb-decision-0087: yes, a one-shot-HTTP adapter can
  satisfy ES-11, by ordering on the read's answer rather than on the request.
  happenstance-neon holds an append until every read its transport dispatched
  earlier has been answered, through the required SqlTransport::reads_settled
  (0.4.0); live, 172 of 1,500 trials red without the fence and 0 of 1,500 with
  it. ES-11 and ES-12 are FROZEN, and the claim is scoped to one transport value.
depends_on: []
related:
  - kb-decision-0087
  - kb-decision-0061
  - kb-decision-0087
  - kb-open-question-es-11-sqlite-ceiling-sample-cost-001
  - kb-open-question-provisional-falsifiers-001
  - kb-decision-0011
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - .kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md
  - crates/happenstance-neon/src/lib.rs
  - spec/SPECIFICATION.md
  - experiments/es-11-fence/results/tally.md
last_reviewed: 2026-10-08
---

# Whether any one-shot-HTTP adapter can satisfy ES-11, now that the first one cannot

## What is true today

`kb-decision-0061` amended ES-11's asynchronous-driver sufficiency condition:
a read must be spawned at its first poll, as before, *and* ordered against a
later append by something the store itself honours. `happenstance-neon` —
the adapter ES-11's own `[PROVISIONAL]` marker named as its predicted
falsifier — does not meet the second half, and the decision records that as a
stated limitation rather than a defect to chase further inside this adapter.
ES-11 keeps `[PROVISIONAL]`; its marker no longer predicts a falsifier, it
records one that already fired.

What that leaves open is a different question from the one the falsifier
answered. It is settled that *this* one-shot-HTTP shape fails. It is not
settled whether *any* one-shot-HTTP shape can pass.

## Why it does not reduce to a fix for this adapter

A pooled driver satisfies ES-11 because both operations — the read's snapshot
and the append's commit — enter one queue at the store, in the client's
issuing order. A one-shot-HTTP transport has no equivalent: each operation is
an independent HTTPS request, and a proxy in front of the store may route them
to different backends with no ordering between the two. The one primitive the
transport offers today is HTTP/2 over a single multiplexed connection, which
`happenstance-neon` already ships and which measurably narrows the race
without closing it. There is no second primitive waiting to be turned on;
closing the gap needs either an ordering guarantee the transport does not
expose today, or an endpoint-side guarantee this decision did not find and
did not go looking for exhaustively.

## What would answer it

Any of three shapes would settle the question, in either direction:

- A one-shot-HTTP adapter that supplies a store-honoured ordering primitive —
  something beyond client-side connection reuse — and passes ES-11 as
  amended. That would show the obligation is meetable by the shape, just not
  by the primitive `happenstance-neon` has today.
- A convincing argument, grounded in what the transport and the endpoints it
  talks to can actually guarantee, that no such primitive can exist for a
  proxy-fronted one-shot-HTTP store. That would turn "open" into "answered:
  no," which is a different clause outcome from either amending ES-11's scope
  or leaving it as a permanent stated limitation.
- `query_items_share_one_snapshot` (ES-12's rule) going red on the same
  adapter, which `kb-decision-0061` already names as a falsifier of its own
  soft-edge figure and would force this question to be revisited alongside
  that one. **This trigger has fired**: the rule went red against live Neon on
  2026-09-28 (PR #20) and in CI run 37504851570 (2026-10-06, PR #36), passing on
  re-run each time. Phase 17's ES-11 record (lane L8) takes the question up.

## Why nothing forces an answer yet

`NEON_CONNECTION` is not a repository secret — no organisation or repository
secret of that name exists — so every step of the `live-neon` CI job is gated
on its presence and the job prints its own "proves nothing" notice and stops
before running anything. Strictness in that job is free today for exactly
that reason. The decision to keep it strict, made in `kb-decision-0061`,
should be re-examined the day someone adds the secret and the job starts
running for real — not before, because there is nothing to observe until
then.

## What this is not

This is not a request to weaken ES-11's scope to exclude one-shot HTTP —
`kb-decision-0061` already declined that path as the weakening the clause's
own text warns against, since ES-11 and ES-12 reduce to ES-10 plus a ceiling
and a scope carve-out there would reach append-condition correctness itself.
It is also not a request for a `Capability` the fixture could decline;
`Capability` states what a fixture can arm, not whether a store provides a
guarantee, and using it here would print "skipped" where the honest word is
either "conforms" or "does not."

## 2026-10-07: answered by ADR-0087, proposed, pending acceptance

**This question is answered, but not closed.** [`kb-decision-0087`](../decisions/0087-es-11-is-met-on-one-shot-http-by-a-read-settlement-fence.md)
(`proposed`) takes the first shape under *What would answer it*: a one-shot-HTTP adapter that
supplies a store-honoured ordering primitive. The primitive is not one the endpoint exposes. It is
the read's **answer**: `happenstance-neon` holds an `append` until every read its transport
dispatched earlier has been answered, and an answer follows the statement's execution, so the
read's snapshot precedes the append's commit at the store.

Measured on the live endpoint under a rule fixed before the first counted run
(`experiments/es-11-fence/results/tally.md:14-25`): without the fence 172 of 1,500 trials red,
every one in the same direction; with it, 0 of 1,500, and both racing rules green in every attempt.
So the answer to the title is **yes, a one-shot-HTTP adapter can satisfy ES-11**, by ordering on
the response rather than on the request.

It stays open until the owner accepts ADR-0087, and the fence it describes is not on `main`: the
spike that measured it is unmerged, and landing it is a separate change (a breaking one, to
`SqlTransport`). On acceptance this atom gains `superseded_by: kb-decision-0087`. Two things the
record leaves unmeasured bear on the answer's reach: a `wasm32` `fetch` transport, where settling a
read from a promise callback is reasoning only, and the order between two handles over separate
transports, which no rule observes.

## Closed — 2026-10-08

**Superseded by `kb-decision-0087`** (phase 17, lane L8, accepted by the owner on 2026-10-08),
which takes the first shape under *What would answer it*: a one-shot-HTTP adapter that supplies a
store-honoured ordering primitive. The primitive is not one the endpoint exposes. It is the read's
**answer**: `happenstance-neon` holds an `append` until every read its transport dispatched
earlier has been answered, and an answer follows the statement's execution, so the read's
snapshot precedes the append's commit at the store.

- **The title's question — can any one-shot-HTTP adapter satisfy ES-11?** Yes. Measured on the
  live endpoint under a rule fixed before the first counted run: without the fence 172 of 1,500
  trials red, every one in the same direction; with it 0 of 1,500, and both racing rules green in
  every attempt.
- **The mechanism.** `SqlTransport::reads_settled`, a required method from `0.4.0` (a
  semver-major break of the trait), with `ReadLedger`, `ReadTicket` and `ReadsSettled` public in
  `happenstance_neon::transport`. Settlement is driven by the transport's own I/O, never by
  polling the read stream, so the rule's one task cannot deadlock itself.
- **What the answer does not reach.** The ordering domain is one transport value: two handles
  over separate transports are not ordered against each other, which no rule observes through the
  port, and the claim states it as its limit. A `wasm32` `fetch` transport settling a read from a
  promise callback is reasoning, not measurement.

ES-11 and ES-12 are `[FROZEN]` with the fence on `main`, and ES-11's sufficiency paragraph names
the read's answer preceding the append's dispatch as the second store-honoured order.
