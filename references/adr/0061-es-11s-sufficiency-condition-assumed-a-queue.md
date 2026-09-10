# ADR-0061 — ES-11's sufficiency condition assumed a queue, and `happenstance-neon` has none

- **Status:** proposed
- **Date:** 2026-09-08
- **Phase:** 12 (the `0.2.0` release pass)
- **Amends:** ES-11's asynchronous-driver paragraph in `spec/SPECIFICATION.md` §3 — a **narrowing** of a sufficiency condition, not a widening of the obligation
- **Records:** that `happenstance-neon` does not satisfy ES-11, as a stated limitation
- **Does not change:** ES-11's MUST, its maturity, its `Rule`, its `Cases`, or ES-12's normative content
- **Evidence:** `crates/happenstance-neon/tests/support/transport.rs:20-102`; `crates/happenstance-neon/src/event_store.rs:965-985`; `crates/happenstance-testkit/src/suite.rs:6008-6065` and `:6112-6167`
- **Supersedes nothing.** It is the first ES-11 escalation to survive the check below; the previous one was retracted at `2e0a0ae`.

## The question

ES-11's `[PROVISIONAL]` marker named the adapter that would falsify it: *"the
first one-shot-HTTP adapter that cannot meet this in one round trip — which is
the outcome to expect."* `happenstance-neon` is that adapter, it exists, it runs
the suite against a live endpoint, and it fails
`read_result_is_stable_under_concurrent_append` intermittently.

**What does the clause owe, now that the predicted event has happened for an
unpredicted reason?**

## First, the check this claim has to survive, because the last one did not

`HANDOVER.md:109-114` records an ES-11 escalation *made in error and retracted*
(`2e0a0ae`), and warns that commit `0341467`'s message asserting the falsifier had
fired is history rather than guidance. Any new claim on this clause is owed a
demonstration that it is not that claim in new clothes.

**The retracted claim was:** no asynchronous driver can satisfy ES-11, because its
first poll can only *start* the round trip that takes the snapshot. It was refuted
by `happenstance-postgres`, which conforms by handing the work to the runtime at
the first poll rather than holding the cursor's opening as an inline future that
advances only while someone polls it.

**This claim is narrower:** a driver with **no shared ordering primitive between
its operations** cannot satisfy ES-11.

The refutation does not touch it, and the reason is not an argument — it is a
measurement. **`happenstance-neon` already does the thing that refuted the old
claim.** `crates/happenstance-neon/tests/support/transport.rs:22-24` puts the
request on the wire at call time rather than holding an inline future, and `:40-46`
states the consequence in the clause's own terms — *"spawning at the first poll
puts the `SELECT` on the wire before `append` is even called, which is the earliest
an adapter is permitted to fix its state."* `:83-102` records that this transport
owns a dedicated runtime, and that the Postgres-shaped `Handle::try_current()`
capture is what it shipped *first* and why that specific shape fails here. The
remedy has been applied and the failure survives it. That is the residual after
the old claim's fix, not the old claim restated.

Two secondary checks agree. The failure is **always in one direction** — the read
sees the append that followed it, never the reverse — which is what a lost
ordering race looks like and not what a paging defect looks like. And the paging
defect ES-11 is mostly about is **unreachable here by construction**: one read is
one statement, buffered whole in one round trip
(`crates/happenstance-neon/src/event_store.rs:965-972`), so there is no second
sample for the answer to drift against.

## What actually fails, and the numbers

Run repeatedly against the live endpoint at `--test-threads=1`, both transport
configurations redden **intermittently**, and HTTP/2 over a single connection
(`pool_max_idle_per_host(1)`) reddens markedly less often than HTTP/1.1 over a
default connection pool.

**This record deliberately quotes no rate, and the reason is a defect in its own
first draft.** That draft carried a table of red-runs-per-twenty and cited
`crates/happenstance-neon/tests/support/transport.rs` as the source — but that
file is a doc comment restating the same table, so the citation resolved to the
claim rather than to evidence. No raw log of the counting session was committed,
and `experiments/` holds a study for every other measurement this project relies
on. The finding does not depend on the rate: the *direction* is what falsifies
ES-11's sufficiency condition, and the direction is invariant. A citable
frequency needs the sweep run again, under `experiments/`, with its output
committed.

The adapter ships the second, and HTTP/2 there is not a performance choice: a
single multiplexed connection is the only ordering primitive the transport offers,
so the proxy at least *receives* two nearly simultaneous requests in the order they
were issued. It does not follow that one backend's snapshot precedes another
backend's commit, and no configuration of this client can make it follow.

## The sentence in ES-11 that is false, which is the part that binds

ES-11 states the conformance path for an asynchronous driver and then states a
sufficiency condition for it:

> A read spawned at its first poll and an append spawned afterwards land in the
> same queue in that order, so the snapshot precedes the append without either
> operation having completed when the poll returned.

**That is not a fact about asynchronous drivers. It is a fact about pooled ones,
stated as though it were the general case.** It is true of
`happenstance-postgres` because a read and an append both enter one `PgPool` —
one queue, one order. It is false of `happenstance-neon` by construction: there is
no queue, there are two independent requests, and the proxy hands each to whichever
backend it likes.

The clause already separates a synchronous driver (the first poll *is* the sample)
from an asynchronous one (the sample is *committed to* at that poll), and
**one-shot HTTP is a third shape its second paragraph does not reach.**

The correction is that spawning at the first poll orders the two operations *at the
client*, and what the clause needs is that the snapshot precedes the commit *at the
store*. The full condition is: **spawned at the first poll, and ordered against a
later append by something the store itself honours.**

**This is a narrowing.** It removes a route to a conformance claim rather than
adding one: an adapter that could previously point at that sentence and infer
conformance from spawn order alone now cannot. Nothing an adapter must do gets
easier.

## Why the obligation is not weakened, and why no capability is minted

Two other ways out were on the table and both are refused here.

**Amending ES-11 to exclude one-shot HTTP from its scope** is the move the clause's
own text warns against thirteen lines further down: *"Neon's inability to meet the
obligation in one round trip becomes an argument for weakening the obligation
rather than for supplying the primitive."* It also reaches further than it looks —
**ES-11 and ES-12 reduce to ES-10 plus a ceiling**, and the thing ES-11 protects is
an append condition whose boundary is derived from the maximum position a read
observed. A widened ES-11 admits a write that should have been rejected, with no
error anywhere. The clause is load-bearing and this ADR does not touch its MUST.

**Minting a capability the fixture can decline** would make the rule report a
*skip*. A `Capability` in this testkit says what a fixture can **arm** — a fault,
a second handle, a reopen — not whether the store under it provides a guarantee.
Turning a normative MUST into an opt-out would print "skipped" where the true
statement is "does not conform", and every adapter that found the rule inconvenient
would have the same door. It is the *"a gate that tolerates a named set is one
commit away from tolerating the wrong one"* defect wearing a different hat.

## The decision

1. **ES-11's asynchronous-driver paragraph is amended** to name the shared ordering
   primitive its sufficiency condition silently assumed. The MUST, the maturity,
   the `Rule` and the `Cases` do not move.
2. **`happenstance-neon` does not satisfy ES-11**, and that is a stated limitation
   of the adapter rather than a defect awaiting a fix. It is already stated in the
   crate's README (`crates/happenstance-neon/README.md:64-81`), in the read
   stream's own documentation (`event_store.rs:965-985`) and in the conformance
   transport (`tests/support/transport.rs:36-82`); this ADR is what makes it a
   decision rather than three files agreeing.
3. **The `live-neon` CI job stays strict.** No named failure is tolerated, no
   allowlist is added. Its cost is currently zero — see below.
4. **ES-11 keeps `[PROVISIONAL]`**, and its marker is rewritten to record a
   falsifier that has *fired* rather than to predict one. What is still open is not
   whether this shape fails — it does — but what a *conformant* one-shot-HTTP shape
   would look like, if one exists.

## ES-12: the same falsifier arrived, and the result is different — with a soft edge

ES-12's marker says its falsifier is *"the same adapter"*. The adapter arrived and
**ES-12's own predicted defect is unreachable on it**: ES-12 forbids an adapter
that issues one statement per `QueryItem` from picking a late event up in a later
statement, and `happenstance-neon` issues exactly one statement per `read`
regardless of item count. `query_items_share_one_snapshot` passes, and every
observed failure in the measured runs is attributed to
`read_result_is_stable_under_concurrent_append` — the `Later` event named in those
failures is that rule's (`crates/happenstance-testkit/src/suite.rs:6047`).

**The soft edge is worth writing down rather than rounding off.**
`query_items_share_one_snapshot` appends *after the first poll* and asserts the
drained set is unchanged (`suite.rs:6150`, `:6164-6167`) — structurally the same
exposure as ES-11's rule at `:6062-6065`, differing only in how many events are
seeded. It was not observed red in sixty measured runs, which at the sibling rule's
observed rate is consistent with either immunity or luck. So the honest conformance
figure for this adapter is **104 of 105 observed**, with a 105th that is exposed to
the same race by construction and has not yet lost it. ES-12's marker is amended to
say that, rather than to keep predicting a falsifier that has already come and gone.

## Four statements the falsifier's arrival made false, repaired here

`cargo xtask spec-trace` checks that citations *resolve*, not that prose is
*current*, so none of these was caught by the gate:

1. §6.5's portfolio, **Transport** row: *"**No.** `happenstance-neon` is a phase-2
   skeleton … which falsifies a signature and is not a far end."* It is a far end
   and it is occupied.
2. §6.5's portfolio, **Position allocation** row says the same of
   `happenstance-postgres`, which has been real and conformant since phase 10b.
3. §3's `EventStore` cell: *"two — transport and completeness — are empty at both
   ends."* Transport is not.
4. ES-11's `Rejects:` bullet ends *"It is conformant today"* — written on
   2026-08-06 at `12ecb1a`, when the adapter it calls hypothetical did not exist,
   and untouched through the two phases in which it was built and shipped.

The clause census does not move: no clause is added, removed, or changes maturity.

## The cost of this decision, stated because it is not what the finding claimed

The `live-neon` job being kept strict was described as costing *"red about 1 run in
40"*. **It costs nothing today.** Every step of that job is gated on
`env.NEON_CONNECTION != ''`, and there is no such secret: `gh secret list` on
`Wet-Ink-Corporation/happenstance` returns nothing at exit 0, `actions/secrets`
reports `total_count: 0`, the repository has no environments, and the only
organisation secret visible to it is `REDKILN_TOKEN`. The job runs its first step,
prints its own *"this job proves NOTHING"* notice, and stops.

So the strictness is free until somebody adds the secret, and the argument for it
is unchanged rather than urgent. What this does change is that the decision to keep
it strict should be re-read on the day the secret lands, not before.

**And a consumer pays nothing either.** The gated tests carry an `#[ignore]`
attribute whose reason names the requirement
(`crates/happenstance-neon/tests/neon_conformance.rs:79`), so `cargo test` on the
published crate never runs them and never silently skips them — they list under
`-- --ignored --list` with the reason attached.

## Consequences

- ES-11's async paragraph gains the missing half of its sufficiency condition;
  its normative content is untouched.
- ES-11's and ES-12's `[PROVISIONAL]` markers stop predicting and start recording.
- `happenstance-neon` ships at `0.2.0` with a stated limitation, which is what
  `[PROVISIONAL]` means: a clause a published crate may fail to satisfy while the
  question it holds is open.
- The honest conformance claim for the adapter is **104 of 105 observed**, and
  every document that states a number for it says so.
- Four stale passages are reconciled in the same change.
- Nothing about the release set moves. Seven crates, as decided at D-04.

## Falsifier

This ADR is reopened by any of:

1. **A one-shot-HTTP adapter that satisfies ES-11.** The narrowed sufficiency
   condition says what it would have to supply — an ordering primitive the *store*
   honours, not merely the client. Neon's own WebSocket driver would be one such
   transport, and building against it would be a different adapter rather than a
   configuration of this one.
2. **`query_items_share_one_snapshot` going red on `happenstance-neon`.** The soft
   edge above becomes a second failing rule, the figure becomes 103 of 105, and
   ES-12's amended marker is wrong about the outcome rather than about the shape.
3. **A measurement showing the failure has a cause this ADR did not name** — a
   client-side reordering, a pool defect, an endpoint guarantee not being used.
   That would make it a bug after all, and this record the second ES-11 escalation
   to have been wrong.

What does **not** reopen it: the rule passing for a while. It is intermittent by
nature, and a green run is not evidence.
