---
id: kb-decision-0061
title: ES-11's sufficiency condition assumed a queue, and one-shot HTTP has none
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0061
reversibility: medium
phase: 10
supersedes: null
superseded_by: null
summary: >-
  ES-11's [PROVISIONAL] marker named the adapter that would falsify it — the
  first one-shot-HTTP adapter that cannot meet this in one round trip — and
  happenstance-neon is that adapter, failing
  read_result_is_stable_under_concurrent_append intermittently. The predicted
  event arrived for an unpredicted reason. ES-11 contains a sentence false as
  written, stated as the general conformance path for asynchronous drivers: a
  read spawned at its first poll and an append spawned afterwards land in the
  same queue in that order. That is a fact about pooled drivers presented as
  a fact about async ones — spawning at the first poll orders the operations
  at the client, while the clause needs the snapshot to precede the commit at
  the store. happenstance-postgres gets both halves because both operations
  enter one PgPool; a one-shot HTTP store gets only the first, because there
  is no queue, there are two requests, and the proxy may serve them on
  different backends. The failure is not the paging defect the clause
  anticipated: the read does not self-paginate, it buffers a whole result
  set in one round trip as ES-12 asks. Four decisions. The sufficiency
  condition is amended to read spawned at the first poll AND ordered against
  a later append by something the store itself honours — a narrowing, which
  removes a route to a conformance claim rather than admitting a shape the
  MUST would reject, so nothing an adapter must do gets easier.
  happenstance-neon does not satisfy ES-11, as a stated limitation. The
  live-neon job stays strict, with no named failure tolerated and no
  allowlist. ES-11 keeps [PROVISIONAL], its marker rewritten to record a
  fired falsifier instead of predicting one. ES-11's MUST, maturity, Rule
  and Cases are untouched, and so is ES-12's normative content. Two ways out
  refused: putting one-shot HTTP outside ES-11's scope is the weakening the
  clause's own text warns against, and ES-11 and ES-12 reduce to ES-10 plus a
  ceiling so a widening reaches append-condition correctness; minting a
  Capability so the rule reports a skip misuses Capability, which says what a
  fixture can arm and not whether a store provides a guarantee, and would
  print skipped where the truth is does not conform. Both transport
  configurations redden intermittently and HTTP/2 over one multiplexed
  connection reddens markedly less than HTTP/1.1 over a default pool; no
  rate is stated, because the first draft's figures cited only a doc comment
  restating them and no raw log was committed, and the finding rests on the
  direction of failure, which is invariant. ES-12 got the same falsifier and
  survived with a soft edge — one statement per read whatever the item count
  makes its predicted defect unreachable — so the honest figure is 104 of
  105 observed, with a 105th exposed and not yet having lost. Strictness
  costs nothing today because NEON_CONNECTION is not a repository secret and
  every live-neon step is gated on it; that should be re-read the day
  someone adds it.
depends_on:
  - kb-decision-0011
related:
  - kb-decision-0051
  - kb-decision-0013
  - kb-open-question-es-11-sqlite-ceiling-sample-cost-001
  - kb-open-question-one-shot-http-es-11-001
  - kb-playbook-repair-frozen-clause-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - .kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md
  - references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - crates/happenstance-neon/src/lib.rs
  - .github/workflows/ci.yml
  - HANDOVER.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
---

# ES-11's sufficiency condition assumed a queue, and one-shot HTTP has none

## Context

ES-11's `[PROVISIONAL]` marker named its own falsifier in advance: "the first
one-shot-HTTP adapter that cannot meet this in one round trip." That was a
deliberate bet, made so the clause would be settled before the freeze rather
than after. `happenstance-neon` is that adapter, and running the conformance
suite against a live endpoint at phase 10b makes
`read_result_is_stable_under_concurrent_append` fail intermittently. The
predicted event arrived — but not for the reason the clause anticipated, which
is why this needed its own decision rather than a checkbox.

This finding had to survive one check first: `HANDOVER.md` records an earlier
ES-11 escalation, made in error and retracted, claiming no async driver could
conform at all — refuted by `happenstance-postgres` conforming through a
runtime handoff at the first poll. This claim is narrower and different in
kind: a driver with no shared ordering primitive between its own operations
cannot conform, which that refutation does not touch, because what fixed
`happenstance-postgres` was giving it one queue, and a one-shot-HTTP transport
has none to give.

## The finding

ES-11 states, as the general sufficiency condition for asynchronous drivers,
that a read spawned at its first poll and an append spawned afterwards land in
the same queue in that order, so the snapshot precedes the append. That
sentence is true of a pooled driver and false, as a general statement, of an
async one. Spawning at the first poll orders the two operations *at the
client*; the clause needs the snapshot to precede the commit *at the store*.
`happenstance-postgres` gets both because both operations enter one `PgPool` —
one queue, one ordering guarantee. A one-shot-HTTP store gets only the first:
there is no queue, there are two independent HTTPS requests, and a pooled
proxy may route them to different backends with no ordering between them.

The failure is not the paging defect ES-11's paragraph was written to catch —
`happenstance-neon` does not self-paginate; it buffers a whole result set in
one round trip, exactly what ES-12 asks for, and that part holds. Both
transport configurations measured redden intermittently, and HTTP/2 over a
single multiplexed connection — the only ordering primitive the transport
offers — reddens markedly less often than HTTP/1.1 over a default connection
pool, without closing the window. No rate is stated here: the only cited
source for an earlier number was a doc comment restating itself, with no raw
log committed, unlike every other measurement under `experiments/`. The
finding rests on the direction of failure, which held across every run, not on
a frequency that was never actually logged.

## Decision

1. **Amend the sufficiency condition** to require a read spawned at the first
   poll *and* ordered against a later append by something the store itself
   honours. This is a narrowing — it removes a route to a conformance claim,
   it does not admit a shape the MUST would otherwise reject, and nothing an
   adapter must do gets easier.
2. **`happenstance-neon` does not satisfy ES-11**, recorded as a stated
   limitation rather than papered over.
3. **The `live-neon` CI job stays strict.** No named failure is tolerated and
   no allowlist is added — `live-postgres` already carries the lesson that a
   gate tolerating a named failure set is one commit from tolerating the
   wrong one.
4. **ES-11 keeps `[PROVISIONAL]`**, with its marker rewritten to record a
   fired falsifier rather than predict one. What stays open is not whether
   this shape fails, but whether any conformant one-shot-HTTP shape exists.

Two alternatives were refused. Scoping one-shot HTTP out of ES-11 entirely is
the weakening the clause's own text already warns against — ES-11 and ES-12
reduce to ES-10 plus a ceiling, so any widening of scope reaches
append-condition correctness itself. Minting a `Capability` the fixture could
decline so the rule reports a skip misuses `Capability`, which states what a
fixture can arm, not whether a store provides a guarantee; it would print
"skipped" where the honest word is "does not conform."

## ES-12's soft edge

`happenstance-neon` issues exactly one statement per `read` regardless of item
count, so ES-12's own predicted defect — a late event picked up only in a
later statement — is structurally unreachable, and
`query_items_share_one_snapshot` passes. It is not immune: that rule appends
after the first poll and asserts the drained set unchanged, which is
structurally the same exposure ES-11 has. The honest figure is 104 of 105
rules observed, with the 105th exposed to the same race and not yet having
lost it — a different statement from proven immune.

## Consequences

ES-11's MUST, maturity marker location, `Rule`, and `Cases` are untouched;
ES-12's normative content is untouched. Strictness in `live-neon` costs
nothing today, because `NEON_CONNECTION` is not a repository secret and every
step of that job is gated on its presence, printing its own "proves nothing"
notice and stopping — a fact the original finding did not have and that
should be re-read the day someone adds the secret, not before. Reopened by a
one-shot-HTTP adapter that supplies a store-honoured ordering primitive and
satisfies ES-11, by `query_items_share_one_snapshot` going red (making the
figure 103 of 105), or by a measurement finding a different cause this
decision did not name.
