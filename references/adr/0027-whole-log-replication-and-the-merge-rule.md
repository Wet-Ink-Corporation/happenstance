# ADR-0027 — Replication is whole-log at 1.0, and the merge rule, bulk ingest and both topologies are ratified

- **Status:** accepted, 2026-10-14. The owner took two calls in the project thread: `wi-53c544`
  option A (*"Whole-log"*) and `wi-0fbcac` option A (*"freeze all 4 with edits"*). Both atoms
  are in `.kb/decisions/`, beside this record's atom,
  [`kb-decision-0027`](../../.kb/decisions/0027-whole-log-replication-and-the-merge-rule.md).
- **Date:** 2026-10-14
- **Phase:** 13 (`happenstance-sync` and its testkit). Issue #106, epic #64.
- **Supersedes:** nothing. It writes down SY-1 – SY-7 as settled and decides the rest of
  the ledger's row for 0027.
- **Door:** one-way. An accepted ADR is immutable, and the clauses it ratifies sit on a crate
  1.0 promises (`happenstance-sync`, ADR-0066 §1).
- **What it is not:** a freeze. No clause marker moves to `[FROZEN]` here (§9).

---

## 1. Context

The ledger reserved 0027 for one question: *how do two logs reconcile — the merge rule, the
compensation contract, and whether hub-and-spoke and peer-to-peer are one abstraction or
two?* It covers SY-1 – SY-7 and SY-19 – SY-31. Much of that was frozen before this record was
written, when the specification took the sync port's shape without waiting for a transport.
What was still open, and what phase 13's work list names (#106), was:

- **Scope.** SY-27 was `[DEFERRED]`: whole-log or scoped? SY-28, the round-trip rule, was its
  consequence and deferred with it. SY-29, the cost policy over a peer-supplied `Query`, was
  `[PROVISIONAL]` and existed only if the answer was scoped.
- **The merge rule's details.** SY-7 (one adjudicator per fact family), SY-10 (both topologies
  first-class, a merge rule per direction) and SY-23 (`EventId` order as the deterministic
  order) were `[PROVISIONAL]`.
- **Bulk ingest.** SY-14 was `[DEFERRED]` against a measurement on the Neon peer.

The 1.0 dispositions gave all of these `freeze-by-13`, and recorded one sequencing hazard: the
instrument SY-27 and SY-28 needed was the suffix- or filtered-subset store CF-27 names, which
phase 14 builds after phase 13.

## 2. What this record takes as settled and does not reopen

- **Ingest never refuses, and a conflict is compensated** (SY-1 – SY-5, `[FROZEN]`).
- **A wire-carried condition is evidence, not an instruction** (SY-6, `[FROZEN]`). LiveStore's
  RFC 0002, *command replay* (`livestorejs/livestore@3cb74e62`,
  `contributor-docs/rfcs/0002-command-replay.md`), reaches the same need from the other side: to
  re-judge a conflict, the adjudicator needs the query the decision was made against. This
  workspace already sends it, as the group's guard, without shipping the command (#247, audit
  finding L-03).
- **The port describes one peer, and hub-ness is not a type** (SY-8, SY-9).
- **Re-delivery is a no-op, deduped on the store-assigned `EventId`, never a content hash**
  (SY-11 – SY-13).
- **One round trip per port method, and an owned resume token** (SY-15, SY-16), and the port
  binds `EventStore` (SY-17).
- **Arrival order is local, and peers need not agree on a total order** (SY-19), with
  transitivity (SY-24, SY-25) and causality as a tag (SY-26).

## 3. Decision 1: replication is whole-log at 1.0 (`wi-53c544`, option A)

A peer offers its counterpart every event it holds, and the port carries no peer-supplied
filter over what is replicated. A slice that must not be replicated, for confidentiality or for
size, is its own store.

### 3.1 The options

- **A. Whole-log for 1.0, with scoped as a later additive opt-in.** Chosen.
- **B. Scoped at 1.0.** Build #122's filtered-subset store now, and put a `Query` and SY-29's
  cost policy on the port. It serves Kestrel Cold Chain's confidentiality slices directly. It is
  also the largest single piece of phase 13. A spoke holding a filtered subset cannot tell
  *"not yet received"* from *"filtered out"*, so a scalar watermark is unsound against it, and
  the resume shape has to become a version vector against a filtered hub. A `Query` on the port
  bounds nothing about its own cost (E2E-40), so SY-29's policy has to be built to stop a buggy
  spoke from making the hub scan its whole log.
- **C. Run SY-27's experiment first, then decide.** What the specification literally said: build
  a filtered spoke and try a position-based resume against it. It is honest, and it is the same
  build as B, paid before anyone knows whether it is needed.

### 3.2 The trade, stated

What A gives up: a fact that belongs to two slices is written to two stores, and a hub's
adjudicator cannot see across slices, because each slice is a separate log with its own
adjudication. What A buys: phase 13 drops its riskiest item, and the sync release does not
wait on a filtered resume.

The prior art is a shipped library on the other end of most of §5's axes. LiveStore
replicates whole logs and partitions data into many small stores; filtered sync is not
supported (#247, audit finding L-21). Its reason is not this workspace's — a LiveStore store is
also its aggregate, and in DCB the consistency boundary is a query, not a store — so it is taken
as evidence that whole-log plus many stores is a production shape, not as a model of the
contract.

### 3.3 What follows

- **SY-27** is rewritten to state whole-log, and moves from `[DEFERRED]` to `[PROVISIONAL]`.
  Its falsifier is the flip condition: a real deployment before 1.0 that needs filtered spokes
  and cannot use one store per slice. A firing is answered by scoped replication as an additive
  opt-in with its own ADR, never by changing what a whole-log peer does.
- **SY-28** asserts whole-log equality: once two peers have pushed and pulled to quiescence,
  each holds the same set of `EventId`s, each byte-identical. It never compares positions, which
  SY-19 says two peers do not share. Both clauses name one rule,
  `round_trip_preserves_the_whole_log`, whose wrong implementation is a peer that withholds the
  events matching a filter.
- **SY-29** leaves the 1.0 path. Under whole-log no port method takes a peer-supplied `Query`, so
  its MUST binds nothing at 1.0. Its disposition becomes `renew-past-1.0`, against the first
  scoped-replication proposal, which must build `ingest_refuses_an_unbounded_peer_query` with
  the `Query` it adds. CF-38 forbids a renewal with no experiment, and this one names it.
  What survives is narrower and is not SY-29's: a pushed guard still travels as evidence (SY-6),
  ingest never evaluates it, and an adjudicator that chooses to run a pushed guard's query
  against its own log is running peer-authored code above the port. Bounding that is the
  adjudicator's job.
- **#122**, the filtered-subset store, leaves the 1.0 path and can close as not planned for 1.0.
  **CF-27's suffix store stays phase 14's**, and the sequencing hazard between phases 13 and 14
  is dissolved: nothing in phase 13 needs that instrument now.
- **`SyncPeer::Resume` stays an opaque associated type.** §5.12 chose it so that the port was not
  frozen against the whole-log answer before the experiment ran. It now does a second job:
  keeping a scoped opt-in additive.
- A future scoped proposal has a cheapest form already on record: exclude events at their origin
  rather than filter at the hub. Positions may have gaps and the watermark is a version vector,
  so that shape is sound for a receiver (audit finding L-22). It would be that proposal's to
  argue, not this record's.

## 4. Decision 2: SY-7 is ratified as written (`wi-0fbcac`, option A)

For each fact family, identified by tag key (SY-35), at most one peer authors a compensation.
Which peer is runner configuration, not a property of the port.

**Why the marker's escape is not available.** SY-7's falsifier asks whether two adjudicators'
compensations can be made convergent, so that duplicates collapse. They cannot, in this
contract. An `EventId` is the pair of the store that wrote the event and the position it
assigned (`crates/happenstance-core/src/identity.rs:96`), and that is frozen (VT-5, SY-12). The
same compensation written on two peers therefore gets two identities and never dedupes. SY-13
forbids the content-hash identity that would rescue it. So every-peer-adjudicates produces *N*
compensations for one conflict, which is the Kestrel Rotor failure the clause was written
against.

**The cost.** While the adjudicator is unreachable, a conflict stays unresolved on every other
peer. If it is lost for good, an operator moves the role in configuration. Under hub-and-spoke
the hub is the natural adjudicator, and decision 1 changes nothing here.

## 5. Decision 3: SY-10 is ratified, with adjudication as its reason

The runner permits a different merge rule in each direction of one edge. The clause's rationale
was Kestrel Cold Chain's edge tier, strictly hub-and-spoke because a spoke's slice was a
confidentiality boundary. Under decision 1 that slice is its own store, so confidentiality no
longer argues for the topology.

The argument that stands is adjudication. The scenarios' merge rule is *first to the hub, not
first to decide* (`references/scenarios/README.md:436`). The hub sees every spoke and
adjudicates what spokes push. A spoke sees only itself, takes what the hub sends without judging
it, and must cope when the hub refuses its push (SY-34, E2E-45). `Watermark` is already a version
vector so that both topologies resume correctly (`crates/happenstance-sync/src/lib.rs:59-66`),
and SY-9 already made hub-ness a property of an edge.

## 6. Decision 4: SY-14's bound is one ingest round trip per pulled batch

The clause read *"a number of round trips bounded independently of the batch size"*. No rule can
assert that, because nothing names the bound. A pull is already page-bounded (SY-15), so the
bound becomes **one ingest round trip per pulled batch, however many of its events are already
held**, which a rule can count.

**The mechanism is already built**, and more of it than the deferral text assumed:

| What | Where |
|---|---|
| The unique `(origin_store, origin_position)` index VT-8 requires | `crates/happenstance-sqlite/src/event_store.rs:294`, `crates/happenstance-cloudflare/src/event_store.rs:210`, `crates/happenstance-postgres/migrations/0001_event_log.sql:92`, `crates/happenstance-neon/migrations/0001_neon_log.sql:117` |
| Held rows skipped inside the write | `crates/happenstance-sqlite/src/ingest_spike.rs:12`, `ON CONFLICT (origin_store, origin_position) DO NOTHING` |
| One call per pulled batch | `IngestStore::ingest`, `crates/happenstance-sync/src/ingest.rs:168` |
| A conditional append in one round trip on one-shot HTTP | `happenstance-neon`, one `SERIALIZABLE` statement |

So the deferral's second open question, *what an already-written adapter pays to add the unique
index*, is answered: nothing. What is left is the clause's own measurement — Kestrel Rotor's
1,840 events over Neon's one-shot HTTP inside a 34-minute window — and the marker stays
`[DEFERRED]` until it passes in phase 13. A failure changes the ingest path's shape or records a
deployment limit. It does not restore the per-event condition VT-8 and VT-10 already ruled out.

## 7. Decision 5: SY-23 is ratified, with a caveat

Where a deployment needs a peer-independent deterministic order, that order is `EventId` order.
The order already exists and is published: `StoreId` and `EventId` derive `Ord` in
`happenstance-core` (`crates/happenstance-core/src/identity.rs:43`, `:96`), and `StoreId` is a
byte array, so the order is the same on every platform.

**The caveat.** `EventId` order is deterministic, and it is neither fair nor temporal. It sorts by
origin store first, so every event of the store whose id sorts lower comes before every event of
the one that sorts higher, whatever happened first in the world. It must therefore never decide
who wins a conflict. That is arrival at the adjudicator (SY-7). A domain that wants *first in real
time* carries a writer's clock in a tag (SY-35) and accepts the skew.

## 8. Considered and rejected

- **Total order plus client rebase** (LiveStore). A backend keeps one total order per log and
  accepts a push only as a fast-forward (`sync-cf/src/cf-worker/do/push.ts:74-78`); a client
  that loses pulls, rebases its pending events and pushes again. Refusing a push by what the
  receiver already holds is the refusal SY-1 and SY-33 forbid, and a single arbiter is the
  central writer SY-10 rules out (audit L-01). A rebase moves an event past facts its append
  condition never saw, so the decision it was made under stops holding without anyone seeing it;
  LiveStore's RFC 0002 names it *"convergence without correctness"*. Rewriting pending local
  history also breaks SY-34 (audit L-02). Compensation keeps both facts and records the decision.
- **Scoped at 1.0** and **experiment first** (§3.1, options B and C).
- **Ratify verbatim** (`wi-0fbcac`, option B). SY-14 would stay untestable as worded, and SY-10
  would rest on a reason decision 1 removed.
- **Convergent compensations from every peer** (§4). Needs content-hash identity; SY-13.
- **A hybrid logical clock** for SY-23. A new field on a published type, which breaks every
  adapter, bought for causality no stated requirement needs.
- **`RecordedAt` as the order.** VT-9 forbids it.

## 9. Markers

The owner's word was *"freeze"*. In this repository a marker moves on evidence in the tree, when
its rule exists: ADR-0066's adversarial pass refuted thirteen of fourteen proposed freezes
(`references/adr/0066-what-1-0-promises.md:200`), and phase 13 calls a freeze without its
instrument *"the decorative kind of freeze"* (`runbook/phases/13-sync.md:160`, in the #122 item this
record strikes). So this record ratifies the clauses, edits their text, and leaves each
marker to the change that lands its rule.

| Clause | Marker before | Marker after | What moves it to `[FROZEN]` |
|---|---|---|---|
| SY-7 | PROVISIONAL | PROVISIONAL (text unchanged; grounding added) | `only_the_adjudicator_compensates` and the two-adjudicator test (#136) |
| SY-10 | PROVISIONAL | PROVISIONAL (rationale replaced) | `directional_merge_rules_compose` (#137) |
| SY-14 | DEFERRED | DEFERRED (bound reworded) | the Neon bulk-ingest measurement and `bulk_ingest_is_idempotent_in_bounded_round_trips` |
| SY-23 | PROVISIONAL | PROVISIONAL (caveat added) | `event_id_order_is_identical_on_every_peer` (#136) |
| SY-27 | DEFERRED | PROVISIONAL (rewritten: whole-log) | `round_trip_preserves_the_whole_log` with a filtering mutant (#140) |
| SY-28 | DEFERRED | PROVISIONAL (rewritten: whole-log equality) | the same rule (#140) |
| SY-29 | PROVISIONAL | PROVISIONAL (falsifier renewed past 1.0) | the first scoped-replication proposal; disposition `renew-past-1.0` |

§7.1's totals move from 26 provisional and 12 deferred to 28 and 10.

## 10. Consequences for documents

- `spec/SPECIFICATION.md`: SY-7, SY-10, SY-14, SY-23, SY-27, SY-28 and SY-29 as above; §5.8's
  preamble; the §1 count and the *expect these to move* entry; CF-27's marker and §6.5's
  completeness row no longer say phase 13 may need the suffix store first.
- `runbook/ledgers.md`: the 0027 row, the open-decisions row, the deferred and provisional
  ledgers, and the 1.0 dispositions for SY-7, SY-10, SY-14, SY-23, SY-27, SY-28, SY-29 and CF-27.
  The sequencing-hazard paragraph is struck as dissolved.
- `runbook/phases/13-sync.md`: #106 ticked; #122 leaves the 1.0 path; #140 rewritten.
- Path-and-line citations into the specification that the edits shifted are repointed in prose
  files. Comments in `.rs` and `Cargo.toml` files that cite specification lines past §5 were not
  touched in this change, which was prose only.

## 11. Falsifiers

- **Decision 1** is falsified by a real deployment before 1.0 that needs filtered spokes and
  cannot use one store per slice. The answer is a scoped opt-in with its own ADR, additive over
  whole-log peers. After 1.0 the same need is met the same way.
- **SY-7** is falsified by two peers that must both adjudicate one family and whose compensations
  can be made convergent; §4 says why this contract cannot, and the two-adjudicator test records
  the divergence as evidence.
- **SY-14** is answered either way by the Neon measurement.
- **SY-23** is falsified by a requirement `EventId` order fails and a store-assigned timestamp
  satisfies; a firing adds a second order and leaves `EventId`'s derived `Ord` alone.

## 12. Evidence

- `/mnt/project-files/notes/phase-13-owner-calls.md`, calls 2 and 3, the grounded analysis the
  owner approved (project notes, not in the tree).
- The LiveStore prior-art audit, findings L-01, L-02, L-03, L-21 and L-22; filed for the tree by
  #247.
- `crates/happenstance-core/src/identity.rs:43`, `:96` — `Ord` on `StoreId` and `EventId`.
- `crates/happenstance-sync/src/ingest.rs:168`, `crates/happenstance-sqlite/src/ingest_spike.rs:12`
  and the four origin indexes in §6.
- `references/scenarios/README.md:436` — *first to the hub, not first to decide*.
