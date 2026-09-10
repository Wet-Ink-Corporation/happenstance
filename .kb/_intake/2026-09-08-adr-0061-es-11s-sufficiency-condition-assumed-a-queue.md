# ADR-0061 is written and wants an atom

**Date:** 2026-09-08
**Kind:** decision record, staged for `/redkiln:kb-ingest`
**Long form:** `references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md`
**Amends:** ES-11's asynchronous-driver paragraph — a *narrowing* of a sufficiency condition
**Does not change:** ES-11's MUST, maturity, `Rule` or `Cases`; ES-12's normative content
**Settles:** the question staged at `.kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md`

## The one question

ES-11's `[PROVISIONAL]` marker named the adapter that would falsify it — *"the
first one-shot-HTTP adapter that cannot meet this in one round trip."*
`happenstance-neon` is that adapter, and it fails
`read_result_is_stable_under_concurrent_append` intermittently. **What does the
clause owe, now that the predicted event has happened for an unpredicted reason?**

## The check this claim had to survive

`HANDOVER.md` records an ES-11 escalation *made in error and retracted* (`2e0a0ae`)
that claimed **no async driver can conform**, refuted by `happenstance-postgres`
conforming through a runtime handoff at the first poll.

This claim is narrower — **a driver with no shared ordering primitive between its
operations cannot** — and what separates them is a measurement rather than an
argument: **`happenstance-neon` already does the thing that refuted the old claim.**
Its conformance transport spawns at call time, states the clause's own sufficiency
sentence back at it, and records that the Postgres-shaped `Handle::try_current()`
capture is what it shipped first and why that shape fails here. The remedy is
applied and the failure survives it, so this is the residual after the old fix
rather than the old claim restated. Two secondary checks agree: the failure is
always in one direction (the read sees the later append), and the paging defect
ES-11 is mostly about is unreachable there because one read is one statement.

## The finding

**ES-11 contains a sentence that is false as written**, stated as the general
conformance path for asynchronous drivers:

> A read spawned at its first poll and an append spawned afterwards land in the
> same queue in that order, so the snapshot precedes the append.

It is a fact about *pooled* drivers presented as a fact about async ones. Spawning
at the first poll orders the two operations **at the client**; the clause needs the
snapshot to precede the commit **at the store**. A pooled driver gets both halves
at once; a one-shot-HTTP driver gets only the first.

Run repeatedly against the live endpoint, both configurations redden
**intermittently**, and HTTP/2 over a single multiplexed connection reddens
markedly less often than HTTP/1.1 over a default pool. HTTP/2 is the only ordering
primitive the transport offers and it narrows the window without closing it.

**No rate is stated, and the atom minted from this brief must not add one.** The
first draft carried red-runs-per-twenty figures whose only cited source was a doc
comment restating the same figures, and no raw log was committed — unlike every
study under `experiments/`. The finding rests on the *direction* of failure, which
is invariant; a citable frequency needs the sweep run again with its output
committed, and that is queued rather than done.

## The decision

1. **Amend the sufficiency condition** to read *spawned at the first poll, **and**
   ordered against a later append by something the store itself honours.* This is a
   **narrowing** — it removes a route to a conformance claim rather than admitting a
   shape the MUST would reject. Nothing an adapter must do gets easier.
2. **`happenstance-neon` does not satisfy ES-11**, as a stated limitation. Already
   true in the crate's README, its read stream's documentation and its conformance
   transport; the ADR is what makes it a decision rather than three files agreeing.
3. **The `live-neon` job stays strict.** No named failure tolerated, no allowlist.
4. **ES-11 keeps `[PROVISIONAL]`**, with its marker rewritten to *record* a fired
   falsifier instead of predicting one. What is open is no longer whether this shape
   fails — it does — but whether a conformant one-shot-HTTP shape exists at all.

**Two ways out were refused.** Amending ES-11 to put one-shot HTTP outside its scope
is the weakening the clause's own text warns against, and ES-11/ES-12 reduce to
ES-10 plus a ceiling, so a widening reaches append-condition correctness. Minting a
capability so the rule reports a *skip* misuses `Capability`, which says what a
fixture can **arm** and not whether a store provides a guarantee; it would print
"skipped" where the truth is "does not conform".

## ES-12 got the same falsifier and survived it, with a soft edge

`happenstance-neon` issues exactly one statement per `read` whatever the item count,
so ES-12's own predicted defect — one statement per `QueryItem`, a late event picked
up in a later statement — is unreachable and `query_items_share_one_snapshot` passes.
**It is not immune, though.** That rule appends after the first poll and asserts the
drained set unchanged, which is structurally ES-11's exposure; sixty measured runs at
the sibling rule's rate separate luck from immunity poorly. So the honest figure is
**104 of 105 observed**, with a 105th exposed to the same race and not yet having
lost it — recorded rather than rounded off.

## The cost is zero today, which the finding did not know

The finding priced strict CI at *"red about 1 run in 40"*. **`NEON_CONNECTION` is not
a secret**: `gh secret list` returns nothing at exit 0, `actions/secrets` reports
`total_count: 0`, there are no environments, and the only organisation secret visible
to the repository is `REDKILN_TOKEN`. Every step of `live-neon` is gated on it, so the
job prints its own *"proves NOTHING"* notice and stops. Strictness is free until
someone adds the secret, and the decision to keep it strict should be re-read on that
day rather than before it. A consumer pays nothing either — the gated tests carry
`#[ignore]` with a reason naming the requirement, so `cargo test` on the published
crate never runs them and never silently skips them.

## Four passages the falsifier's arrival made false, repaired in the same change

`spec-trace` checks that citations *resolve*, not that prose is *current*, so the gate
caught none of them: §6.5's **Transport** row still called `happenstance-neon` a
phase-2 skeleton and "not a far end"; its **Position allocation** row said the same of
`happenstance-postgres`; §3's `EventStore` cell said transport was "empty at both
ends"; and ES-11's `Rejects:` bullet ended *"It is conformant today"*, written
2026-08-06 (`12ecb1a`) and untouched through the two phases in which that adapter was
built and shipped. **The general lesson is the one worth keeping: a falsifier firing
invalidates prose that names the axis, and nothing mechanical looks for it.**

## Falsifier

Reopened by a one-shot-HTTP adapter that satisfies ES-11 by supplying an ordering
primitive the *store* honours; by `query_items_share_one_snapshot` going red on
`happenstance-neon`, which makes the figure 103 of 105 and ES-12's amended marker
wrong about the outcome; or by a measurement finding a cause this ADR did not name — a
client-side reordering, a pool defect, an endpoint guarantee unused — which would make
it a bug after all and this the second ES-11 escalation to have been wrong. **Not**
reopened by the rule passing for a while: it is intermittent, and a green run is not
evidence.
