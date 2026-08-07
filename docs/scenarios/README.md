# Scenario catalogue

Six deployments, designed to sit at the far end of six different axes, and each
walked line by line against the contract as it exists on disk today. This file
is the catalogue: what each scenario is, which axis it instruments, its
vocabulary, its decisions, its projections, its sync topology, and the moment it
goes wrong. [`E2E-CASES.md`](E2E-CASES.md) turns the breakages into test cases.

## Why scenarios at all

CLAUDE.md states the standing rule these were built to serve:

> A port is only as well-designed as the *spread* of what implements it.
> `MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object all
> serialise their writers and assign positions under a lock — four adapters, one
> storage shape.

The same argument applies one level up. Four adapters of one storage shape is a
narrow spread; one worked example of one decision is a narrower one. The
`course-subscriptions` example runs for forty milliseconds, makes two decisions,
and never replicates, never rebuilds a read model, never deletes anything and
never meets a store that cannot hold a transaction open. Every one of the 27
conformance rules builds a fresh store, appends three to five events, and asserts
something about what comes back (`crates/happenstance-testkit/src/suite.rs`).

So the ports have been checked against a population that agrees with them. These
six scenarios are the population that does not.

## How to read a scenario

Each was designed first and then walked against the code by a second reader whose
only job was to find where the writeup had lied — to itself, about the API, or
about arithmetic. The corrections are folded in. Where a decision was demolished
it is marked **cut** and the reason given; where it was merely wrong about *why*
it was hard, the corrected finding replaces the original. Nothing here is
presented as standing if it did not survive.

Two things the walk consistently found, worth saying before the scenarios:

**The write half is in better shape than any scenario expected.** Every query and
every append condition in all six scenarios — thirty-odd of them, including
seven-type single-tag items, four-item cross-namespace boundaries and deliberately
untenanted uniqueness checks — is expressible verbatim against `QueryItem::new`,
`Query::from_items` and `AppendCondition::new`
(`crates/happenstance-core/src/query.rs:57-77`, `:164-179`;
`crates/happenstance-core/src/append.rs:63-88`). DCB's query language did not need
extending once. Superset tag matching (`crates/happenstance-core/src/tag.rs:231-245`)
does real work in every scenario and needed no secondary index anywhere.

**The read half, the replication half and the lifecycle half are not.** Those
three are where all six converge, and they converge on a small number of the same
holes.

## Housekeeping

Three unrelated companies in this catalogue are called Kestrel. They are
distinguished throughout by their second word — **Kestrel Cold Chain**,
**Kestrel Motor**, **Kestrel Rotor**.

---

## The six, and the axis each instruments

| # | Scenario | Axis it sits at the far end of | What the workspace has at that end today |
|---|---|---|---|
| 1 | [Kestrel Cold Chain](#1--kestrel-cold-chain--refrigeration-field-service) | **Completeness** — a store holds a strict subset of the log its conditions range over | Nothing |
| 2 | [Wattline](#2--wattline--ev-charging-at-4200-tenants-on-one-log) | **Position order vs visibility order** | `happenstance-postgres`, planned, unbuilt |
| 3 | [Norvant Pharma](#3--norvant-pharma--twenty-views-two-stores-one-log) | **Plural readers** — many independent projections over one log | Nothing; `ProjectionStore` has never had a consumer |
| 4 | [Kestrel Motor](#4--kestrel-motor--a-uk-claims-log-at-seven-years) | **Time** — the operations that only exist because years passed | Nothing |
| 5 | [Turnstile](#5--turnstile--a-stadium-onsale-run-entirely-at-the-edge) | **Nothing can be held open** — no session, no cursor, no surviving caller | `happenstance-neon`, planned, unbuilt |
| 6 | [Kestrel Rotor](#6--kestrel-rotor--offshore-wind-spares-between-a-depot-and-a-vessel) | **N independently authoritative writers** to one fact set | Nothing; every adapter is the sole authority for its own log |

Two of the six axes already have a named instrument in CLAUDE.md
(`happenstance-postgres` for position allocation, `happenstance-neon` for the
one-shot transport). Four do not, and the catalogue's first structural finding is
that the workspace names instruments for the axes it has already noticed.

---

## 1 — Kestrel Cold Chain — refrigeration field service

### The domain

Kestrel Cold Chain Services maintains commercial refrigeration and HVAC for
supermarket estates across northern England and Scotland: 2,412 contracted sites,
about 31,000 refrigeration assets, 138 field engineers across four regions, and
roughly 1,900 work orders a day. A P1 "food at risk" call carries a four-hour SLA
and a £750 contractual penalty, so all the commercial pressure lands on the first
two hours of a job — hours spent in basement plant rooms, rooftop enclosures and
−22 °C cold stores where there is no data signal. Each engineer carries a rugged
tablet holding a genuine on-device SQLite event store: their own writes in full,
plus a hub-pushed 90-day slice for the sites on their rolling schedule. Devices
sync to one Cloudflare Durable Object per region; the four hubs feed a Neon
Postgres estate store over one-shot HTTP, with a Ladybug graph beside it for F-Gas
audit traversals. The physical world is the part nobody can roll back: by the time
a conflict is detected the compressor is brazed into the pack and the system is
under charge.

### The axis

**Completeness.** Everything in the workspace — `MemoryEventStore`, the 27 rules,
the worked example, every planned adapter — assumes a store holding the complete
log its queries range over. `Query::all()` means "everything that happened". Here
a spoke deliberately holds a slice, chosen for size and for commercial
confidentiality: Scottish subcontractors must not hold Yorkshire's parts pricing,
so a spoke's slice is a confidentiality boundary as well as a storage one. A DCB
decision is then made against an append condition whose query ranges over events
the device does not have and cannot know it does not have.

The 90-day prune happens entirely outside the port — `EventStore` has two methods
and neither of them deletes (`crates/happenstance-core/src/store.rs:117-145`) — and
after it the store passes all 27 rules unchanged. Including
`query_all_matches_every_event` (`suite.rs:70-80`), whose contract is
store-relative by wording and therefore accidentally correct. The gap is not that
the contract asserts completeness. It is that **a partial store is
indistinguishable from a complete one at every seam an ingest or a runner can
see.**

Two lesser axes ride along and are worth naming because no other scenario carries
them. Decision-to-durability latency: everywhere else in the repository a
condition is evaluated microseconds after the read, and here it is four hours and
one irreversible brazing joint later. And the adjudicating store is *itself*
partial — the Yorkshire hub holds Yorkshire, not the estate — so re-adjudication
happens on a second incomplete log rather than on the truth.

### Vocabulary

Work orders: `WorkOrderRaised` `{wo, site, asset, priority}`, `WorkOrderAssigned`
`{wo, eng}`, `WorkOrderReassigned` `{wo, eng, from-eng}`, `EngineerArrived`,
`DiagnosticRecorded`, `WorkOrderCompleted` `{wo, site, asset, eng, outcome}`,
`WorkOrderDeferred`, `WorkOrderCancelled`.

Serialised parts: `PartClaimed` `{unit, part, pool, epoch, wo, eng, van}`,
`PartClaimReleased`, `PartFitted`, `PartScrapped`, `PoolStockReceipted`,
`PoolStockWrittenOff`.

Fungible stock: `VanStockReceipted` / `VanStockConsumed` / `VanStockAdjusted` /
`VanStockTransferred`, all `{van, part}`.

Refrigerant: `CylinderAssigned` / `CylinderReturned` `{cyl, van, gas}`,
`RefrigerantCharged` / `RefrigerantRecovered` `{wo, asset, cyl, gas}`,
`LeakTestPassed` / `LeakTestFailed`.

Hub-authored compensations: `PartClaimSuperseded` `{unit, epoch, supersedes,
wo, eng, by-eng, by-wo}`, `WorkOrderCompletionContested` `{wo, kept-eng,
contested-eng, kept-outcome, contested-outcome}`, `StockReconciliationRequired`
`{van, part, sign}`.

The tag design is load-bearing and is not a stylistic choice. **Everything
replication must reason about is in the tags.** The hub runs a domain decision —
it adjudicates a contested claim and authors a compensation — and it does so
without ever parsing a payload, because every fact the decision needs is in the
`EventType`, the `Tags` and the condition `Query` that travelled with the batch.
That is ADR-0003 surviving at the exact point most likely to break it, and it
survives only because the vocabulary was built for it.

**Not an event:** the per-peer confirmation watermark ("the Yorkshire hub has
acknowledged everything I wrote at or before local position 288,447"). It is
device-local sync state, it changes on every sync, and turning it into an event
would put a write in the log for every heartbeat. It nonetheless has to be
readable inside a projection's transaction, and there is no seam for that. See
`today_schedule` below.

### Decisions

**D1 — ClaimPooledUnit.** Claim serialised unit `SC-2024-77341` from the Yorkshire
pool for work order `KES-2026-441897`.

```rust
let query = Query::from_items([
    QueryItem::new(
        ["PartClaimed", "PartClaimReleased", "PartFitted", "PartScrapped"],
        Tags::from_pairs([("unit", "SC-2024-77341")])?,
    )?,
    QueryItem::new(
        ["PoolStockReceipted", "PoolStockWrittenOff"],
        Tags::from_pairs([("pool", "YORKS"), ("part", "CPL-ZB58KCE-TFD-551")])?,
    )?,
])?;
```

Item 1 is this individual unit's entire life; item 2 establishes it is in this
pool at all and has not been written off. The fold walks item 1 in position order:
the unit is free iff the newest event is absent or is `PartClaimReleased`, and
`epoch = count(PartClaimReleased) + 1`.

The condition is deliberately position-free:

```rust
AppendCondition::new(Query::from_item(QueryItem::new(
    ["PartClaimed"],
    Tags::from_pairs([("unit", "SC-2024-77341"), ("epoch", "4")])?,
)?)?)   // after: None
```

*Why the epoch tag.* The obvious condition — the unit-life query with
`after: last_seen` — is position-relative, and `SequencePosition` is meaningless
outside the store that assigned it (`crates/happenstance-sync/src/lib.rs:64-69`).
Worse, `AppendCondition` derives `Serialize` behind the `serde` feature and will
cheerfully put `after: 288446` on the wire
(`crates/happenstance-core/src/append.rs:117-123`), where it is a number with no
referent. Epoch-tagging converts the position boundary into a tag boundary and
makes the condition replicable verbatim. Note that the position-free form is what
`AppendCondition::new` gives you by default (`append.rs:65-70`) — the API already
makes the replicable shape the easy one.

*The correction.* The scenario originally claimed the epoch trick fails silently
under a pruned slice: an engineer whose 90-day window has lost the epoch-3 release
derives epoch 3, conditions on a different tag, and both appends succeed with the
invariant unenforced. **That does not follow.** If she under-counts releases by
one she guards `PartClaimed{unit, epoch:3}` — and the hub *holds* the epoch-3
claim, because that claim is what the release she cannot see was releasing. Her
append is rejected and adjudicated. The scheme degrades into a **spurious
conflict**, not a silent double-claim. To get a silent double-claim the count has
to be wrong by more than one in a particular direction, which is constructible but
was not constructed. The completeness axis is still the right target; this
particular demonstration of it does not hold, and E2E-CASES states the corrected
version.

**D2 — CompleteWorkOrder.** One eleven-type item tagged `{wo}`; condition on
`{WorkOrderCompleted, WorkOrderDeferred, WorkOrderCancelled}` tagged `{wo}` with
`after: None`. Naturally position-free, because a work order terminates once in
its whole life and the condition can quantify over the whole log.

The hard case is the two-engineer job. A lead and a mate both hold the work order
on their own tablets, both in the same plant room, both offline for the same three
hours. The lead completes it `outcome:repaired`; the mate, who does not know,
completes it `outcome:parts`. Both locally valid; jointly two terminations with
different money and different SLA stop times.

*The correction.* The scenario claimed the hub "cannot pick by outcome without
decoding a payload, which ADR-0003 forbids the peer". That contradicts its own
vocabulary: `WorkOrderCompleted` carries `outcome:repaired` **as a tag**, and the
hub-authored `WorkOrderCompletionContested` carries `kept-outcome` and
`contested-outcome`, which the hub could only have obtained from tags. The outcome
is available to a payload-free peer. The premise is removed. What survives is
sharper and is the reason the decision stays in the catalogue: **an append
condition does nothing across a partition** — neither store holds the other's
write, so no condition on either tablet could have prevented this — and
adjudication is therefore necessarily a hub-side *domain* decision. That is a
replication-topology finding, not a boundary finding, and the writeup originally
framed it as the latter.

**D3 — ConsumeVanStock.** A signed sum over ~340 events for one SKU on one van. A
counting invariant must be conditioned on the events it counted, so the honest
condition is the query with `after: last_seen` — precisely the form that cannot
cross a store boundary. Kestrel's answer is to append this class
**unconditionally** and detect breaches after the fact: a hub projection folds each
van's SKU balance and emits `StockReconciliationRequired` when the sum goes
negative.

The rule that falls out is the best thing in this scenario and is domain-independent:

> A counting append condition is only sound offline behind an **exclusive-writer
> grant**. An existence condition is only sound offline if the store holds the
> whole history of the thing whose existence is asserted.

*The correction.* The scenario filed the rule as a library breakage —
`AppendCondition` looks identical either way. That is an ergonomic observation,
and the domain's own answer (unconditional append, hub-side reconciliation) works
against the port exactly as it stands (`store.rs:144` takes
`Option<&AppendCondition>`). The rule deserves an ADR. It does not deserve to be
filed as something the library breaks on. What *is* a real ergonomic hole is that
`append(&events, None)` is syntactically identical to a forgotten condition, and
this domain has one class of each.

**D4 — ChargeRefrigerant. Cut.** The decision claimed `after: last_seen` was sound
because `CylinderAssigned` grants the cylinder exclusively to one van, so "there
is no second store whose positions could disagree". But D5 says every group
carries its origin condition and the hub re-reads it against its own 4.1M-event
log — **the hub is the second store.** `after: 288455` interpreted in hub numbering
names an unrelated recent event, so the check runs over an arbitrary recent tail
and passes vacuously (`append.rs:95-107` compares raw positions). The decision does
not demonstrate a subtle soundness argument living outside the type system; it
demonstrates an unsound one, and the writeup did not notice. It is cut. Its
residue — that the only enforceable ingest rule available today is "reject any wire
condition carrying `after: Some(_)`", and that this rule is too blunt — survives as
a case.

**D5 — IngestPeerBatch.** The hub's own command, run inside the Yorkshire Durable
Object. Not one query: each group in the push carries the `AppendCondition` its
origin decision used, and the hub re-reads that condition's
`fail_if_events_match` against its own log. For an accepted group the origin
condition is used verbatim; for a violated group the hub appends
`[losing_event, PartClaimSuperseded]` as **one batch** under a different condition
keyed on the `supersedes:` tag — the idempotence guard for the compensation, not
the invariant guard.

`read_decision_model` is generic over `EventStore`, the flavour with no `Send`
bound (`store.rs:198-208`), so this compiles and runs unchanged inside the DO.
That would be the first thing in the repository to exercise ADR-0001's motivating
claim rather than assert it.

Three things break at once, and all three are real.

1. **`EventStore::append` takes exactly one condition per batch**
   (`store.rs:141-145`), so a 39-event push decomposes into seven appends and the
   device's unit of work does not survive the boundary. A crash mid-ingest leaves
   the hub holding four of seven groups. That is *correct* DCB — the boundary is
   the query, not the batch — and it is stated nowhere in the contract, the ADRs
   or the 27 rules.
2. **Ingest is not idempotent and cannot be made so.** `Event` is
   `(event_type, data, tags, metadata)` and `SequencedEvent` is
   `(position, event)` (`event.rs:183-188`, `:277-282`) — nothing globally unique.
   On first delivery the hub accepts a group and writes `PartClaimed{unit,
   epoch:4}`; on re-delivery after a dropped socket the same condition now matches
   the hub's own copy, returns `ConditionViolated`, and the hub routes into the
   violated branch — **superseding the event it just accepted.** The scenario's own
   guard protects the compensation, not the accept path. This is the single
   sharpest finding in the scenario.
3. **The hub is running a domain decision**, which means `happenstance-sync` cannot
   be a domain-free pipe. It can still be a payload-free one, which is the ADR-0003
   result above.

*The correction.* The 17:00 burst arithmetic was dropped. 35 reconnecting vans ×
~9 groups is 315 tag point-lookups against an indexed table, and 20 appends/sec is
nothing; SQLite in a Durable Object does that in milliseconds against a 30-second
ceiling. Presenting it as a cliff weakened the writeup, because the DO cliff that
*is* real — a 4.1M-event `pool_availability` rebuild with no chunked lifecycle to
express it — was then stated almost in passing.

### Projections

Four, down from six. `estate_compliance` restated `pool_availability`'s chunking
on a different transport and `asset_touch_graph` was a store variation rather than
a demand variation; both are cut. The four that remain carry four genuinely
distinct lifecycle demands.

**`today_schedule` (device SQLite) — needs non-event state.** Nine to fourteen
rows, and the load-bearing column is confirmation state: confirmed /
unconfirmed-*N*m / superseded. That column cannot be computed from events at all.
It needs the per-peer watermark, which is device-local sync state.

*The correction.* The scenario claimed there is "no seam" for it. There is one: the
sync runner can `begin()` on the same `ProjectionStore` and
`commit(batch, &ProjectionId::new("sync/yorkshire"), local_position)` — the
watermark *is* a local position, so it type-checks against `projection.rs:109-114`.
What genuinely does not work is (a) a *generic* sync runner writing into a
*generic* `Batch`, because `type Batch<'a>` carries no trait bounds
(`projection.rs:80-83`) and generic code can therefore `begin`, `commit` and
`rollback` a batch and cannot put a row in it; and (b) that the schedule rows and
the confirmation column then live in two transactions that can disagree at an
instant. That is the real finding and it is narrower and sharper.

**`van_stock` (device SQLite) — needs reset, several times a day.** Folds
last-event-wins over local position. Ingest appends at the tail, so a replicated
`PartClaimReleased` whose origin time is 08:44 lands *after* a locally-written
`PartFitted` at 11:20, and the model reports a fitted compressor as available.
Nothing errors, the checkpoint advances, and the read model will offer that
compressor to the next engineer who asks. Recovery is reset-and-rebuild over
~11,000 matching events, and **the port cannot do it**: `commit` takes
`position: SequencePosition`, not `Option`, so a checkpoint can never be returned
to "never run" — a state `checkpoint` can *report* (`projection.rs:93`) and no
method can *produce*. `Batch` carries no bounds, so generic code cannot delete the
read-model rows either. RUNBOOK:69 files reset as an open phase-2 question; this
scenario reclassifies it from administrative to operational, on 138 devices.

**`fgas_ledger` (device SQLite) — must never be reset.** The device's event slice
is pruned at 90 days while the regulatory ledger must span five years, so a
rebuild would silently truncate a compliance record. It must be idempotent instead
— the escape hatch the port's own module documentation names
(`projection.rs:28-30`) — keyed on the event's origin identity, which does not
exist.

*Named honestly:* this makes a read model the system of record for a regulated
five-year ledger whose backing events were pruned at ninety days. That is a domain
design problem, not evidence about the port. What it *does* prove, merely by
coexisting with `van_stock` in the same store, is that reset belongs per
`(store, ProjectionId)` rather than being a store-wide operation.

**`pool_availability` (SQLite inside the Durable Object, wasm32, !Send) — rebuild
is unsurvivable.** Must stay within a few hundred milliseconds of ingest, because
it is what the next device's slice is cut from: a stale hub projection manufactures
the very conflict it exists to prevent. A rebuild here is 4.1 million events and
cannot complete inside one DO request. It would have to be chunked across many
invocations with the checkpoint doing the bookkeeping — and the checkpoint is one
`SequencePosition` and nothing else. There is no second field, no status, no
shadow checkpoint, so during the rebuild the projection reports a checkpoint that
lies about the read model's completeness. The honest cheaper alternative is to
rebuild into a second `ProjectionId` and swap, which the port already permits and
nothing documents.

### Sync topology

Both, at two tiers, and the domain forces it. The edge tier is strictly
hub-and-spoke — 138 tablets to four regional Durable Objects, never to each other
— and the binding reason is commercial rather than physical: a spoke's slice is a
confidentiality boundary and peer-to-peer would dissolve it. The upper tier is
hub-to-hub: four DOs to one Neon Postgres estate store.

The transports have nothing in common (a WebSocket with a resumable cursor versus
one-shot HTTP with no connection) and neither do the ingest policies (edge ingest
re-adjudicates and may author compensations; estate ingest is unconditional
archival that must never reject anything). **Two seams, therefore: transport and
ingest policy.** RUNBOOK:80 already suspects this; this deployment is the evidence.

Note that the `!Send` peer sits in the *middle* of the chain, not at a leaf. The
ingest path must therefore be written against `EventStore` throughout — the weaker
bound, per CLAUDE.md rule 4 — and any runner that reaches for `SendEventStore` to
get a `tokio::spawn` excludes the hub from its own topology.

### The moment it goes wrong

Two engineers, twenty-two miles apart, both offline, both claim the same
compressor within eight minutes of each other under byte-identical conditions.
Both slices show it free. Both decisions are correct; each store enforced its
boundary exactly as the specification requires. There is one compressor.

The damage does not wait for the sync. The first engineer collects it and by 11:20
it is brazed in, evacuated and charged. The second learns by telephone at 11:05
that it is gone — but her tablet still says *Reserved to you*, and because it does,
she does not place a hub order against the noon courier cut-off, which was the one
action that would have saved the day.

The hub then has exactly three options and all of them are the design decision:

- **Reject** her event and hand back a violation. Her device deletes a fact she
  acted on. That is a product failure and also a lie, because the claim really
  happened, and Kestrel's engineers will stop trusting the reservation screen
  inside a week.
- **Accept both.** The hub log says two people hold one compressor, and the
  contradiction propagates into every downstream projection, each of which invents
  its own tie-break and disagrees with the others.
- **Admit her claim as history and, in the same atomic append, write
  `PartClaimSuperseded`.** One `append(&[losing, superseded], Some(&guard))`,
  guaranteed all-or-nothing by `store.rs:130-133` and conformance-tested by
  `append_is_atomic` (`suite.rs:385-403`).

The third is the only survivable one, and it works against the contract as it
stands with no new seam. The merge rule is **first to the hub, not first to
decide** — chosen for stability, not justice, because any rule permitting a later
arrival to displace an earlier acceptance means the hub can say yes and then
retract, which is the same product failure one tier up. The engineer who decided
first can lose. The only lever on fairness is sync frequency.

Then the second failure, quieter and worse. Among the 214 events her device
ingests is a hub-origin `PartClaimReleased` from 08:44, which lands at local
position 288,491 — *after* the local `PartFitted` at 288,462. `van_stock` folds
last-event-wins over local position and now reports a fitted compressor as
available. Nothing errored. The checkpoint advanced.
`positions_are_strictly_monotonic` and `positions_are_unique` both still pass
(`suite.rs:336-365`). **The suite is measuring the property that survived and not
the one that broke.**

---

## 2 — Wattline — EV charging at 4,200 tenants on one log

### The domain

Wattline is a charge point management system: the software an EV charging operator
runs to authorise sessions, manage each site's electrical envelope, and settle
roaming and fleet contracts. 4,200 operators share one Postgres cluster and one
event table, from a national operator with 31,000 connectors down to a hotel chain
with six. A weekday produces 1.1 million charging sessions; the log passed 2.9
billion events in June. The evening peak sustains 1,900 session starts a minute.
About 1,180 of the larger sites also run an on-premises controller with a local
SQLite store, because dynamic load management has to keep allocating amps when the
backhaul drops — so the topology is a Postgres hub with 1,180 partially-connected
spokes that write while offline.

### The axis

**Whether an event's position order is also its visibility order.** Every adapter
in the workspace assigns positions under a lock it holds until commit, so an event
visible at position *N* implies every event below *N* is visible — and that
implication is what makes `AppendCondition::after` sound. Postgres with `nextval()`
does not provide it: a position is taken before the work and released at commit, so
a transaction that started later can become visible earlier.
`AppendCondition::is_violated_by` (`append.rs:95-107`) then answers `false` for an
event the caller genuinely never saw, and the one mechanism DCB has for enforcing a
consistency boundary silently stops enforcing it — on a store that passes all 27
rules, because every rule tests position *values* and none tests position
*visibility* (`suite.rs:336-365`).

*Credit where it is due.* This is not a discovery. `docs/evaluation/revised-runway.md`
already names `nextval()` allocating outside the transaction, the projection that
checkpoints past an event it will never see, and the `xid8` + `pg_snapshot_xmin`
remedy along with its long-transaction stall. This scenario should be credited for
what survives subtracting the ledger, and that is three things: **the per-item read
bound**, **the four-last-seen collapse**, and **the offline-lease argument for
unconditional ingest.**

### Vocabulary

`ChargePointRegistered` `{serial, chargepoint, tenant, site}` — note the `serial`
tag stands alone as the uniqueness key, deliberately not scoped by tenant.
`ChargePointTransferred` `{serial, chargepoint, tenant}` — the *previous* tenant is
in the opaque payload, because two `tenant:` tags on one event would make it match
both operators' queries. `ConnectorCommissioned` / `Decommissioned`,
`CircuitDefined`, `CircuitRatingChanged`, `LoadShedApplied` / `Released`,
`PowerAllocationChanged` `{tenant, site, circuit, session}` — written by the load
controller on every arrival, departure and retune. `SessionStarted` and
`SessionEnded` each carry the same eight tags `{tenant, site, circuit, connector,
session, token, driver, fleet}`, because every one of them is a correlation some
decision folds on and a fold only sees events its own query selected.
`MeterValuesRecorded` `{tenant, session}` — deliberately *not* tagged with circuit
or connector, so 40 million telemetry events a day stay outside every decision
boundary. `TokenIssued` / `TokenRevoked`, `FleetContractDefined`,
`FleetConcurrencyCapChanged`, `FleetPeriodClosed`, `SiteLeaseGranted`,
`SessionSettled`, `RoamingSessionAttributed`.

`ChargePointTransferred` carrying the previous tenant in `Event::data` rather than
as a second `tenant:` tag is ADR-0003 paying off in a case its own worked example
does not cover: the payload is carrying a value that would be *harmful* as a tag,
not merely inconvenient. And keeping 40 million daily telemetry events outside
every boundary costs nothing precisely because the contract refuses to interpret
tag keys.

### Decisions

Three, down from six. `EndSession`, `ApplyLoadShed` and `CloseFleetPeriod` are all
read-fold-append-conditioned-on-the-same-query at different sizes, which
`course-subscriptions` already covers and which the contract does not behave
differently about at 828,000 events than at three. `ApplyLoadShed` survives as
*positive* evidence rather than as a decision — see below.

**D1 — StartSession.** Four tag families in one boundary: circuit, connector, fleet,
token.

```rust
Query::from_items([
    QueryItem::new(["CircuitDefined","CircuitRatingChanged","LoadShedApplied",
                    "LoadShedReleased","PowerAllocationChanged","SessionStarted",
                    "SessionEnded"],
        Tags::from_pairs([("tenant","northgate"), ("circuit","cir-a3")])?)?,
    QueryItem::new(["ConnectorCommissioned","ConnectorDecommissioned",
                    "SessionStarted","SessionEnded"],
        Tags::from_pairs([("tenant","northgate"), ("connector","conn-88104")])?)?,
    QueryItem::new(["FleetContractDefined","FleetConcurrencyCapChanged",
                    "FleetPeriodClosed","SessionStarted","SessionEnded"],
        Tags::from_pairs([("tenant","northgate"), ("fleet","flt-ridgeway")])?)?,
    QueryItem::new(["TokenIssued","TokenRevoked","SessionStarted","SessionEnded"],
        Tags::from_pairs([("tenant","northgate"), ("token","tok-4f2a91")])?)?,
])?
```

Four invariants at once, one of which — active sessions carrying `fleet:` at most
the fleet's concurrency cap, counted across every site in the tenant — is the one
no aggregate can hold. Under aggregates this is four boundaries and a saga with
four compensations, one of which has to un-deliver electricity to a van. The
boundary expresses cleanly and needs no contract change to *draw*. It needs one to
*bound*, and that is the finding:

**The fleet item is unbounded and cannot be bounded.** `ReadOptions.from` is one
`Option<SequencePosition>` for the entire read
(`crates/happenstance-core/src/query.rs:226-234`) and `EventStore::read` applies one
`ReadOptions` to the whole `Query` (`store.rs:117-121`). The four-item query needs
`from = <FleetPeriodClosed position>` for item 3 and `from = None` for items 1, 2
and 4, because `CircuitDefined`, `ConnectorCommissioned`, `TokenIssued` and
`FleetContractDefined` all predate any recent snapshot. Setting `from` at all
silently drops them: `rated_kw` folds to absent, the connector's commissioning
vanishes, token validity is unknown. Setting it to `None` reads 828,000 events.
There is no third option and no compile error — the handler just decides on a fold
with holes in it.

*This is stronger than the scenario's own version of it.* The writeup introduced
`FleetPeriodClosed` as a watermark and then complained that "the correctness of
every start now depends on a snapshot event". The real problem is that the
workaround for the unbounded read is itself unimplementable.

And the second-order consequence: a handler that works around it by issuing four
separate reads holds four last-seen positions, and `AppendCondition` has one
`after` (`append.rs:52-60`) applied to every item of `fail_if_events_match`
(`:95-107`). To stay sound it must condition on `min(...)`, which re-admits every
event above that minimum for **all four items** — so the quiet token item's stale
boundary is what the busy circuit item gets checked from.

*The correction.* The scenario also presented a "torn read" as a second,
independent hazard: item 1 read before a commit and item 4 after it. **On a store
where visibility order agrees with position order, a lazily-chunked forward read
cannot miss a matching event** — anything committing after the cursor has passed
lands *above* the cursor and is still ahead of it. The tear is real only when an
adapter evaluates a query's items as separate statements in separate snapshots,
which is precisely the Neon shape, and which the scenario never said. Presented as
written it double-counts one defect. Restated correctly it is a distinct and
genuine breakage: **the contract never says whether the items of one `Query` are
evaluated against one snapshot.**

*One further correction.* The fleet-cap invariant does not need a consistency
boundary by the scenario's own account: the contract bills every session above cap
at a penalty tariff, so a billable overage is a tolerated state with an agreed
price, not an invariant. The decision that works hardest to justify an
828,000-event read is the one whose own contract says the item can be dropped. It
is kept here because the *mechanism* it exposes is real regardless of whether this
particular invariant earns it.

**D2 — RegisterChargePoint.** A physical unit sends its first boot notification and
claims a serial.

```rust
AppendCondition::new(Query::from_item(QueryItem::new(
    ["ChargePointRegistered", "ChargePointTransferred", "ChargePointDecommissioned"],
    Tags::from_pairs([("serial", "ABB-T184-9F0C21")])?,
)?)?)   // after: None
```

Note what is absent: there is no tenant tag. The boundary is the whole log, all
4,200 operators. This is the uniqueness case classic event sourcing handles badly —
there is no aggregate whose identity is "the set of all serials" without inventing
a singleton every registration in the estate serialises behind — and DCB expresses
it as a condition on a query matching at most a handful of events out of 2.9
billion. It compiles verbatim; `append.rs:39-49` uses the same construction as its
own worked example, and `QueryItem::new` accepts a tag-only item because it
requires only that *one* of types or tags be non-empty (`query.rs:69-71`).

It is also the decision that forbids store-per-tenant and punishes
tenant-as-partition. A Postgres adapter that partitions by `hashtext(tenant)` — the
obvious move at 1.4 TB — turns this into a 64-partition scatter-gather on every
boot notification. `Query` and `QueryItem` expose only types and tags
(`query.rs:103-110`) and a `Tag` is an opaque string with `key()`/`value()` offered
as convention rather than structure (`tag.rs:88-98`), so nothing an adapter can
read off a query distinguishes "this omits the partition key deliberately" from
"by accident".

*Correctly scoped:* the fix is **adapter configuration, not a contract change.**
The Postgres adapter is told which tag key is its partition key at construction and
inspects `QueryItem::tags()` for it. That works today. Recording it matters so it
is not mistaken for a port defect.

**D3 — TransferChargePoint.** A unit is decommissioned by one tenant and
re-deployed under another, keeping its serial. Two items: the serial's registration
history, and any open session on the charge point. The boundary spans two tenants,
which no other decision in the domain does — and it is the counterexample to every
tidy answer about tenancy. Store-per-tenant cannot express it at all, because the
two halves live in different logs and there is no cross-store append.

Tag-as-tenant cannot enforce one tenant per event either.
`Tags::from_pairs([("tenant","a"), ("tenant","b")])` succeeds and yields a
two-element set: deduplication is on the whole `key:value` string
(`tag.rs:258-265`) and `Tags` offers no `get(key)` to make the ambiguity visible at
read time (`tag.rs:198-256`). Leave it — the DCB specification treats tags as
opaque strings and enforcing key-uniqueness would put a convention in the contract
— but `Tags`' documentation should say the convention is unenforced and name this
case, because a reader of `Tag::key_value` reasonably assumes otherwise.

### Projections

**`site-board` (Postgres)** — one row per connector, 310,000 rows, serving the
operator console and a public availability API. Deliberately cross-tenant, subscribed
by type only. This is the projection the visibility hole damages from the read side:
it checkpoints 100, never sees 99, and there is no mechanism by which it ever finds
out. The repair after a six-day decode bug is "reset and replay *this tenant only*"
— and the checkpoint is one position per `(store, ProjectionId)`
(ADR-0007:100-104), so a per-tenant scope has no representation. Minting one
`ProjectionId` per tenant would give it one, at the cost of contradicting the
projection being deliberately cross-tenant and turning one read model into 4,200
checkpoints over the same table.

*The correction.* The scenario called the missing tenant tag a "data leak between
operators' consoles" and filed it as evidence about the port. It is not. The query
is `QueryItem::of_types([...])` with no tag constraint, so a missing `tenant:` tag
cannot leak across a boundary the query does not draw. It leaks at the row write, in
the read model, where the projection has no tenant value to key on. Application
defect, correctly outside the port.

**`fleet-usage` (Postgres)** — the source of every fleet invoice, and the clearest
case in the catalogue of a *projection* lifecycle operation taking the *write* path
down: a 210-million-event backfill for a migrating tenant opened a 40-minute
transaction that froze the visibility watermark for every handler in the cluster.
Chunking it into bounded transactions is the mitigation, and `begin`/`commit` is
the whole port surface, so chunk size is the runner's business and every adapter's
default is wrong for this.

**`revenue-recognition` (Postgres)** — the projection that decides the failure
policy. A tenant emitted a `SessionSettled` whose payload carried a trailing space
in a currency code, and the decoder rejects it. Halting is *correct* here — a
revenue read model that skips an event is worse than one that stops — and it is
*wrong* for `site-board`, where a stale availability board is worse than one
missing a connector. **So the policy cannot be one policy.** It has to be per
projection, which means it belongs on the `Projection` trait rather than on the
runner. That is a genuine contribution to RUNBOOK:73, which currently asks the
question as though one answer existed.

`roaming-graph` (Ladybug) is retained only for one observation: adding
`RoamingSessionAttributed` to its query means every prior event has to be re-read,
because the events the old query never selected are exactly the ones the new read
model needs. Per ADR-0007 a projection nominates its events with `Query`, which is
the right call and makes a query change indistinguishable from a rebuild.

### Sync topology

Hub-and-spoke operationally — site controllers sync to the hub, never to each
other, because two sites share no electrical infrastructure and have nothing to
merge. But the *invariants* are not shaped that way: a fleet's concurrency cap
spans fourteen sites belonging to one tenant, so the state two spokes contend over
is genuinely shared even though the spokes never speak. That is exactly the case
`happenstance-sync`'s own documentation says is not a special case of peer-to-peer
(`sync/lib.rs:45-51`).

The scenario's one original contribution to the ingest question, and it is a good
one: **`SiteLeaseGranted`.** The hub grants a site *N* fleet slots for the day. Under
that split, ingest does not have to re-check anything — connector occupancy is
edge-authoritative (nobody but that controller can start a session on that
connector, so a replayed `SessionStarted` cannot conflict with anything, ever), and
fleet concurrency is hub-authoritative but the lease already bounded the damage
*before* the events were written. That is evidence for unconditional
append-of-facts-already-decided from a domain where re-checking is impossible
rather than merely expensive.

Note that this **contradicts** Kestrel Cold Chain, which needs ingest to re-check
and compensate. The contradiction is the point and is carried forward as an input
to the architectural specification.

### The moment it goes wrong

At 18:11:04.112 a session start takes position 99 and then spends 26 ms on its
condition probe, because the fleet item intersects a great many tag rows. Seven
milliseconds later a second handler takes position 100, probes a single connector
tag in 1.2 ms, and commits at .128. The first commits at .147. **For nineteen
milliseconds the log has 100 and does not have 99.**

A handler read the boundary inside that window. `read_decision_model` returned
`last = 100` (`store.rs:206` takes the last observed position). It appended with
`after: Some(100)`. When 99 became visible the store evaluated
`is_violated_by(99, ..)`, found `99 <= 100`, and returned false. Two active sessions
on one connector. No error, no rejected append, no failing test.

`site-board` has the same hole from the other side: it checkpointed 100 atomically
with its read model, will next read `from: 101`, and will never apply 99. The
availability feed says the connector is free. It was found three weeks later by a
technician dispatched to a charger a customer was already using.

The remedy — read only below the oldest in-flight transaction — converted a silent
corruption into a loud one. On the second Friday it was live, a projection backfill
opened a 40-minute transaction and froze the visible watermark. Every handler in
the cluster then read a decision model ending at a fixed position *W*, appended with
`after: W`, and was rejected by everything that had committed since — **including its
own previous attempt.** Retrying could not help, because the retry re-read the same
frozen watermark. 4,200 tenants could not start a session for 38 minutes.

Both failures are one fact seen twice: on this store, position order and visibility
order are different orders, and `AppendCondition::after` is sound only where they
agree.

---

## 3 — Norvant Pharma — twenty views, two stores, one log

### The domain

Norvant runs GDP-regulated cold-chain pharmaceutical distribution from 41 depots
across 14 countries: refrigerated trailers, hundreds of thousands of consignments
in flight, 900 handhelds and 41 depot terminals. The authoritative log is Postgres;
each depot runs a Cloudflare Durable Object holding SQLite as its regional hub;
each handheld holds an on-device SQLite store and works offline for hours at a time
in customs yards and mountain lanes. Raw 60-second telemetry never enters the log:
devices summarise it into 15-minute windows, because a sensor feed is not a fact
worth a consistency boundary and the excursion is.

*Arithmetic health warning.* The scenario's own numbers do not close. 1,400 active
trailers summarising into 15-minute windows is ~134,000 window events a day against
a claimed 9,000; 1,400 trailers at 33 pallet positions loaded and unloaded once is
~92,000 pallet events a day against a claimed 25,000 total; and 340,000 in-flight
consignments against 37,000 events a day means the average consignment emits one
event every nine days. The load-bearing figures — the 8M-event log, the 22,000
events/s rebuild, the 16.7-minute backfill — are roughly an order of magnitude low.
**Read the log as ~80M events**, which is the materially harder read-side test and
the one this scenario exists to be.

### The axis

**Plural readers.** Everything built or written so far is about writes: the worked
example is one decision, one boundary, one append; 26 of the 27 conformance rules
are about what a single reader sees or what a single appender may do, and the
twenty-seventh is about two writers. `MemoryEventStore` is an oracle for write
semantics. `ProjectionStore` is provisional by its own admission — *"a port without
a conformance suite is a guess"* (`projection.rs:3-11`) — and it has never had a
consumer at all, let alone a plural one.

This is the plural consumer, deliberately built so that the cheap answer (one
fan-out read) and the safe answer (twenty isolated tasks) are in direct opposition,
because the port adjudicates neither.

*The correction.* Twenty projections exercise about **seven** distinct port
questions. Six of them are the same exercise repeated: type-only `Query`, one
batch, one checkpoint, occasional rebuild. Naming seven and defending each is a
stronger instrument than twenty with six duplicates, because a port's shape is
decided by its extremes and the middle contributes nothing. The seven, and the
projection that carries each:

| Port question | Carried by |
|---|---|
| Reset, in place, under a query change | `excursion_register` |
| Refusal of reset — reset would be a hazard | `audit_trail_export` (`Query::all()`, a hash chain the regulator already holds) |
| Narrow-query checkpoint lag | `cold_chain_certificate_expiry` (~40 matching events a day) |
| Read-your-own-uncommitted-writes | `exception_queue`, `vehicle_contact_graph` |
| Expensive `apply` forcing runner topology | `disruption_blast_radius` (~90 ms per event) |
| Tag-constrained on-device backfill | `pallet_inventory_by_depot` |
| A projection that writes back into the log | `scan_gap_monitor` |

### Decisions

Five, and all five compile verbatim — the three-item `LoadPalletOntoTrailer`
boundary spanning trailer, pallet and vehicle tag namespaces; the
backwards-limit-1 excursion read; the mixed tagged/untagged `SuspendLane` query.

**D1 — LoadPalletOntoTrailer.** Three tag namespaces in one boundary. Occupancy must
not exceed the declared pallet positions; the pallet must not already be on another
trailer; the trailer must not be sealed; the vehicle's certification must not have
lapsed.

*The correction.* The certification clause is correctly diagnosed and wrongly filed.
No append condition in any event-sourced system can guard a clock, because nothing
in the log changed. The domain answer is a scheduler that emits
`VehicleCertificationLapsed` at the boundary, after which the clause is an ordinary
event predicate and DCB works. Listing it as a difficulty implies some port shape
could fix it. None can.

**D2 — OpenTemperatureExcursion.** At most one excursion open per consignment. The
decision model is a fold to a single boolean over a set that only grows, and
`ReadOptions::new().backwards().limit(1)` is exactly right and the only shape that
keeps it O(1). That is a conformance-tested read option
(`read_backwards_from_with_limit`, `suite.rs:306-329`) being load-bearing for
production behaviour rather than for the specification's worked example, and
`MemoryEventStore` implements it correctly — it reverses *before* truncating, so the
limit applies to the newest matches (`memory.rs:163-179`).

The trap sits next to it. `read_decision_model` hardcodes `ReadOptions::new()`
(`store.rs:205`) so this read cannot use the helper, and the helper derives its
boundary from `events.last()` (`:206`) — which on a **backwards** read is the
*oldest* match. A caller copying the idiom onto `backwards().limit(1)` must know to
take `.first()`. Nothing stops them, and the failure is not a compile error: it is a
condition that starts at the first excursion of the consignment's life and rejects
far more often, which under a 240-excursion chiller failure looks like contention
rather than a bug.

**D5 — SuspendLane.** One tagged item on `lane:`, one type-only item for the
deliberately untagged `NetworkFreezeDeclared` / `Lifted`. Semantically right and
expressible: untagged events match a type-only item and cannot match any tagged
one, because `Tags::contains_all` is a subset requirement (`query.rs:113-116`,
`tag.rs:231-245`).

*The correction.* The scenario claimed this decision "would catch" an adapter that
scans where it should seek, pointing at `MemoryEventStore::read` evaluating
`query.matches` over every event before applying `from` (`memory.rs:159-174`). That
is a category error. `memory.rs:25-27` states outright that the oracle is not built
for scale, and **no conformance rule can catch an adapter that scans when it should
seek** — complexity is a benchmark, not an assertion, and a suite that tried would
be asserting on timings. What the decision genuinely motivates is a benchmark
harness the workspace does not have, which is a useful thing to say and not a thing
to say about conformance.

### The seven port questions, stated as findings

**Incremental consumption does not exist.** `EventStore::read` returns
`impl Stream` and the contract crate exports exactly one consumer, `collect`, whose
own documentation forbids the use every rebuild here requires: *"Convenient for
tests, small reads... Do not use it to replay an entire log"* (`store.rs:148-152`).
There is no `for_each`, no chunked take, and no `futures-util` dependency — an
omission `store.rs:154-156` states is deliberate. So an application either buffers
the whole log in a `Vec` or hand-rolls `poll_next` with `core::pin::pin!`. Twenty
projections doing this is twenty hand-rolled stream drivers, and ADR-0007's own
`pump` sketch cannot be written without one.

**There is no way to hold twenty projections together.** ADR-0007's `pump`
signature *does* compile against today's GAT, including holding the batch across
the apply loop, because `apply` is a caller-supplied higher-ranked closure
(`F: FnMut(&mut P::Batch<'_>, &SequencedEvent)`, ADR-0007:62-67). What does not
exist is a heterogeneous registry: `Projection::Store` is an associated type, so a
Postgres projection and a Ladybug projection cannot share a list — `error[E0271]:
type mismatch resolving <NetworkTopology as Projection>::Store == Pg`. The central
premise (one runner, twenty views, two stores) is at minimum two supervisors with
no common type, and every runner is monomorphic in a concrete `Batch`. `Batch<'a>`
carrying no bounds (`projection.rs:80-82`) is what forces that: nothing generic can
write to a batch, so nothing generic can be a projection.

**The batch is not tied to its receiver.** `commit(&self, batch: Self::Batch<'_>,
..)` elides a fresh method-level lifetime never tied to `&self`
(`projection.rs:109-114`). A batch opened on one store instance can be committed to
a *different instance of the same type*: `depot_de.begin()` followed by
`depot_it.commit(batch, ..)` is accepted. With 41 same-typed depot stores and a
runner holding a map of them, mis-committing one depot's inventory writes under
another's checkpoint is a type-correct program. The module doc at `:24-26` claims the
batch "borrows from the store because a transaction cannot outlive its connection",
which is true of the lifetime's *duration* and not of its *identity*. Tying it —
`async fn commit<'a>(&'a self, batch: Self::Batch<'a>, ..)` — costs nothing and
converts a silent cross-store write into a borrow error.

**The `Send` flavour forces the runner topology.** `#[trait_variant::make(
SendProjectionStore: Send)]` (`projection.rs:70`) puts the batch in `commit`'s and
`rollback`'s parameter lists, so the `Send` flavour transitively requires
`Batch<'a>: Send`. `rusqlite::Transaction` is not `Send` and Ladybug's synchronous
handles will not be either, so the on-device SQLite projection store and every
graph projection can implement only the bare flavour and cannot be
`tokio::spawn`ed. The one thing the scenario says is *not* a free choice — runner
topology, forced by a 90 ms `apply` — is forced onto `LocalSet` / `spawn_blocking`
for the graph projections while the Postgres ones spawn normally. Nothing to fix in
the port; this is `trait_variant` behaving correctly. But the module documentation
should say it, because it decides the topology and is currently discoverable only
by writing the adapter and being told.

**"Never run" is unrepresentable, and the wrong reset looks right.** `commit` demands
a `SequencePosition` backed by `NonZeroU64` (`projection.rs:109-114`,
`event.rs:118-119`), so the `None` that `checkpoint` can return cannot be written
back. `commit(empty_batch, id, SequencePosition::FIRST)` is the closest available and
it is wrong in a way nobody will notice: the runner reads `Some(1)`, computes
`1.next()`, and **skips event 1 permanently**.

**Checkpointing past unmatched events is legal, undocumented, and therefore
adapter-defined.** The port never says whether `commit`'s `position` must be a
position the batch actually applied. The narrow projections need the permissive
reading. The scenario asserts it is impossible; it is not — `commit` accepts any
`SequencePosition`, and head is one
`read(&Query::all(), ReadOptions::new().backwards().limit(1))` away, and both
compile. But nothing documents it as legal, no rule pins it, and **an adapter that
validated `position` against what the batch wrote would be equally conformant.** Two
adapters can disagree and both pass. That is the real defect, and it is smaller and
sharper than the impossibility claim.

**Read-your-own-uncommitted-writes is unspecified.** Three of the twenty projections
are correct only under it — the exception queue looks up a route it wrote when it
first saw the consignment, the co-exposure graph finds which other consignments are
currently on a trailer, both possibly written earlier in the same chunk. The port
neither promises nor denies it; the module doc's only guidance is the idempotence
escape hatch (`projection.rs:28-30`), which does not address visibility. A SQLite
transaction gives it; a write-behind Ladybug batch that buffers Cypher until commit
does not; both satisfy the trait. **So the co-exposure graph's determinism failure is
not a chunk-size bug, it is an unspecified port semantic**, and a rebuild against a
different adapter produces a different graph. Rebuild determinism is not a
conformance property anywhere in the workspace.

**No upper bound on a read.** `ReadOptions` has `from`, `backwards` and `limit`
(`query.rs:224-234`) and no `to`. A backfill worker cannot be given the closed
window [1, *H*] while a tail worker owns (*H*, ∞), and `limit` is a count, which
`event.rs:94-96` explicitly forbids treating as a position bound because gaps are
permitted. The backfill must stream past its own horizon and discard client-side.

**The decode failure has no representable error.** ADR-0007's callback is typed
`FnMut(&mut P::Batch<'_>, &SequencedEvent) -> Result<(), P::Error>`, so a decode
failure — not a projection-store failure — must either be forged into the adapter's
`#[non_exhaustive]` error enum or panicked. There is no quarantine, no skip, no
dead-letter, and no way for the pump to report "stopped at position *P* because of
the event at position *P*".

**The skip primitive already exists and nobody has noticed.** `begin()` followed
immediately by `commit(batch, id, poison_position)` applies nothing and advances the
checkpoint past the poison event, atomically, with the port exactly as written. What
does not exist is any way to record that it happened — no status, no quarantine, no
marker — so the read model is permanently wrong for one consignment with nothing
saying so, and the next rebuild hits the same event again.

**And, underneath all of it:** the authoritative store is Postgres allocating
positions by `nextval()`, and neither `EventStore` nor any of the 27 rules says that
once you have observed position *P*, nothing below *P* will appear later. Every one of
the twenty checkpointing projections is unsound on the store the scenario names as
authoritative. `MemoryEventStore::last_position` exists as an *inherent* method only
(`memory.rs:109-111`), so no generic code can even ask.

### Dropped

`alternate_path_index`'s complaint — that it cannot read `network_topology`'s read
model because "they are separate `ProjectionId`s at different checkpoints" — is
self-inflicted. They are both in Ladybug over overlapping queries and the fix is one
`ProjectionId`, one query, one batch. The port refuses nothing. A real version needs
two projections that genuinely cannot share a store, and then it is the cross-store
skew ADR-0007:100-104 already documents as inherent.

### The moment it goes wrong

Ops ships two changes in one deploy: a twentieth projection, and a widened `Query`
on `excursion_register` to include an event type live since February and ignored.

The new projection backfills and converges; nobody notices. The widened query is the
problem: its checkpoint is at the head and every event its new query needs is behind
it, so **the widened query is a lie until the read model is rebuilt**. There is no
reset. So the night-desk operator does what the runbook says: truncate the two
tables, then move the checkpoint back. Two statements, two connections, because
`ProjectionStore` has no seam spanning DDL and a checkpoint. The truncate commits at
02:46:31. A rolling restart takes the pod at 02:46:33. The checkpoint move never runs.

At 02:47:40 the runner returns, reads the old checkpoint, resumes from the next
position, applies 61 events into an empty table, and reports healthy.

At 03:12 a depot chiller fails and fourteen consignments start drifting. The register
records all fourteen. It looks right.

At 03:18 the operator sees fourteen rows where there should be thousands, works out
what happened, and resets properly — truncate, then `commit(empty_batch, id, FIRST)`,
because `commit` takes a `SequencePosition` and not an `Option`, so "never run"
cannot be said. The runner reads `Some(1)`, computes `1.next()`, and skips event 1
forever. Nobody will ever find that.

At 03:21:43 the rebuild reaches a `TemperatureWindowSummarised` from a spare handheld
still on old firmware, still emitting the payload shape whose decoder fallback was
deleted in July as dead code — true going forward, false going backwards. A division
by zero panics the task.

Nineteen projections keep running. **That is the one thing that works**, and it works
because each has its own store, its own batch, its own checkpoint and its own task —
ADR-0007's associated type earning its keep. Nothing alerts, because the `JoinHandle`
goes into a set nobody drains.

At 04:00 the shift board renders and shows zero open excursions. The fourteen drifting
consignments are on no screen. The board cannot tell it is stale: it can read the
checkpoint, and there is no `head()` on `EventStore` to compare it against, and no
status on `ProjectionStore` to ask.

One method missing, three symptoms — a narrow projection cannot advance past events it
examined and did not match; an operator cannot distinguish "behind because nothing
matched" from "dead"; and a dashboard cannot say how stale it is.

### What this scenario confirms rather than falsifies

Worth stating, because it is the property that makes polling work and it is the
strongest argument against adding a tail seam. `ReadOptions::from` is an inclusive
lower bound, not a seek, and positions may have gaps — so `checkpoint.next()` is a
correct resume point even when position *checkpoint*+1 does not exist, and
`read_from_is_inclusive` (`suite.rs:262-276`) is what pins it. Because backfill and
tail are the same loop at different rates, **there is no moment at which one hands over
to the other and therefore no catch-up race at all.** A subscription seam would create
that race where none exists today — and it would be unimplementable on the one-shot
HTTP peer that most needs it, because a subscription returns a stream that must stay
open.

---

## 4 — Kestrel Motor — a UK claims log at seven years

### The domain

Kestrel Underwriting writes 410,000 private-car and light-commercial policies and
notifies about 68,000 claims a year. Its claims log went live in April 2019 and by
August 2026 holds 53.1 million events across 476,000 claim files. The deployment is
four-tier: 214 field engineers carry on-device SQLite stores holding the forty-odd
claims each is working; **every open claim has its own Durable Object**, so there are
hundreds of thousands of independent `SequencePosition` spaces; a Postgres book of
record ingests all of them into one merged log; and the compliance console and the
annual skilled-person auditor reach that same Postgres over one-shot HTTP. A Ladybug
graph holds the fraud read model.

### The axis

**Time, and specifically the operations that only exist because time passed.**
Everything else in the workspace tests a log at a moment. The port is closed under
exactly two operations — append and read — and a seven-year regulated log demands
four more:

1. **Erase a scattered subset** (a data subject's events, across every claim).
2. **Destroy an aged cohort** (the retention schedule).
3. **Rebuild a derivation** from a log that has changed shape underneath it.
4. **Forbid all of the above for a named set** while it proceeds around them (a
   preservation notice).

None of the four is an append. None is a read. And — this is the part nothing else
tests — **the conformance suite affirmatively passes on a store that has performed
all four**, because gaps are permitted (`event.rs:92-97`) and immutability is never
stated.

### Vocabulary

`ClaimRegistered` `{claim, policy, incident}` with three payload versions across seven
years; `ClaimReserveSet`, `ClaimClosed`, `ClaimReopened`, `RepairAuthorised`
`{claim, repairer}`, `PaymentIssued` `{claim, payee}`, `PaymentReversed`,
`VehicleInspected`, `SalvageCategoryAssigned`, `EngineerNoteAdded`.
`WitnessStatementRecorded` `{claim, subject}` — **`subject:` is the tag that cannot be
erased.** `LegalHoldPlaced` / `Lifted` `{claim, hold}`, `RetentionDue`,
`RetentionPurgeExecuted`, `RetentionPurgeRefused`, `ErasureRequested`,
`ErasureExecuted`, `ErasureRefused`, plus policy and repairer lifecycle events.

The v1/v2/v3 payload divergence on `ClaimRegistered` never reaches the sync layer: a
Durable Object forwards a 2019 payload to Postgres byte-for-byte with no schema
agreement and no possibility of a re-encode corrupting it (`event.rs:159-166`,
ADR-0003). **Seven years of payload drift is precisely the case that justifies the
decision, and this is the first scenario in the workspace that spans enough time to
exhibit it.**

### Decisions

Four commands compile and are, mostly, the design working. Two cannot be written at
all.

**D1 — RegisterClaim.** Two items: uniqueness on `{policy, incident}`, in-force check
on `{policy}`.

*The correction, and it is the scenario's headline claim.* It argued that the
uniqueness arm needs `after: None` while the in-force arm needs `after: last_seen`,
and `AppendCondition` has one `after`. **It does not need two.** The handler reads the
same two-item query, sees a 2019 `ClaimRegistered` in its own decision model, and
refuses the command in process — the condition never has to catch a *past* event, only
a *concurrent* one. `after: last_seen` catches exactly the concurrent case: two
notification handlers both read, both see nothing, both take `last_seen` at the latest
policy event, and the second's condition sees the first's `ClaimRegistered` at a
position past `last_seen` and rejects. This is the pattern `define_course` uses
verbatim (`examples/course-subscriptions/src/main.rs:83-104`) and it is the reason a
single `after` is **sufficient rather than a compromise**.

The real breakage in that decision is the one the scenario reaches second: once the
2019 event is purged, the *read* returns nothing and the in-process refusal passes
vacuously. That is a deletion problem, not an `AppendCondition` shape problem, and
stating it as the latter misdirects the fix.

Superset tag matching does real work here for free. `ClaimRegistered` carries
`{claim, policy, incident}`; the in-force arm asks only for `{policy}` and matches it,
and the uniqueness arm asks for `{policy, incident}` and matches it too. No secondary
index, no per-arm event shape, no duplicated events (`tag.rs:231-245`).

**D2 — AuthoriseRepair.** Three arms: the claim's financial history, the repairer's
network status, the hold and fraud state. Three things that would be three aggregates,
in one query and one condition. This is the case DCB exists for and the type system
takes it without complaint.

*The correction.* The retry-storm numbers do not follow. A repairer carrying ~2,300
live authorisations is not 2,300 in-flight command handlers; with `after: last_seen`
only handlers whose read-to-append window straddles the suspension are rejected, which
at a desk handler's decision rate is a handful — and their rejection is the *correct*
outcome, because they decided against an approved-network fact that is no longer true.
The complaint that `Query` has no negation and no payload predicate is accurate
(`query.rs:113-116`) and does not produce a storm here.

**D3 — IssuePayment.** The BACS instruction is a side effect outside the store; the
append is the record that it happened. `EventStore::append` is an `async fn`
(`store.rs:141`) so its future can be dropped — a payments operator closing a browser
tab is routine — and `AppendError` has three variants, none meaning "the outcome is
unknown" (`error.rs:148-166`). On a pooled rusqlite path a dropped `JoinHandle` does
not cancel the blocking closure: the COMMIT executes and the caller is told nothing.

*The correction.* The scenario names the wrong saving mechanism. It says "DCB would
save this if the batch's own events matched the condition query". `MemoryEventStore`
checks the condition against `stored` **before** extending (`memory.rs:197-221`), so
the batch is explicitly not matched against itself — and if it were, `RegisterClaim`
would self-reject. What actually makes the retry safe is either re-reading (the handler
then sees the payment and declines) or reusing the *original* condition object, whose
`after` predates the first payment. The second is undocumented and is worth a sentence
in `AppendCondition`'s docs. The scenario found the right hole and described a
different one.

**D4 — PlaceLegalHold.** `after: None` is correct here and unusual: a purge from any
time at all disqualifies the hold, and a prior hold makes this one redundant.
`AppendCondition::new(query)` gives exactly that shape and `after_opt(None)` is a
first-class const method rather than a special case (`append.rs:63-88`).

What makes it structurally interesting is that it runs on the one-shot HTTP adapter —
read-decide-append collapses into a single conditional statement — while the purge
batch runs against **the same logical log** through the pooled adapter. Two adapters,
one store, no shared session state, and the only thing standing between them is
`AppendCondition`. Nothing in the workspace exercises that: every conformance rule
builds one store from `factory()` and drives it through one handle
(`suite.rs:596-638`), so an adapter whose condition check is correct only within its
own session — a cached max position, a per-connection snapshot, a pool-scoped advisory
lock — passes all 27.

**D5 — ExecuteRetentionPurge. Cannot be written.** `EventStore` has two methods and
neither deletes. There is no seam that says "delete these positions in the same
transaction as this conditional append", so a conformant adapter must go around its own
port — at which point the atomicity the port spent its whole design defending is not
defending this. And the purge is the one operation whose *success* degrades every
future reader: after it, `Query::all()` returns a log with holes no rule detects, and
`positions_are_unique` and `positions_are_strictly_monotonic` still pass, because gaps
are permitted. **The conformance suite affirmatively blesses the state that makes every
subsequent decision unsound.**

*One self-inflicted wound worth naming.* The scenario's own compliance artefact,
`RetentionPurgeExecuted{positionsDeleted: 63, lowest: …, highest: …}`, repeats exactly
the mistake `event.rs:92-97` warns about — *"Never compute `end - start` and call it a
length"*. A purge marker built that way is the false compliance record its own
invariant text warns against.

**D6 — ExecuteErasure. Cannot be written.** Two phases, where phase two's shape depends
on phase one's result, so the two reads are not one snapshot. A hold placed between them
is invisible to the condition.

*The correction.* The scenario overstates the read limitation. A head position *is*
obtainable (`Query::all()`, `backwards()`, `limit(1)`) and an as-of-*P* read *is*
obtainable (`from(P).backwards()`, then reverse in memory —
`memory.rs:163-174` shows `from` acting as an upper bound under `backwards`). The
two-phase read can be pinned. What is genuinely missing is a **forward read with an
upper bound**: at 211 events, buffering and reversing is nothing; on a 53-million-event
backfill it is fatal. And a backwards-from-*P* read is only a snapshot on a store whose
positions are visibility-ordered, which returns us to axis 2.

Beyond that, the erasure touches no event-store method at all — the shred is a key
destruction and a mapping-row delete — so **the log is recording a fact it cannot verify
and cannot enforce.**

### Projections

**`claims-desktop`** — rebuild after the shred, and it poisons. The erased subject's
payloads become undecryptable ciphertext, so `Projection::apply` receives a decode
failure. The failure is **permanent and intended** — the key was destroyed on purpose —
so "retry" and "dead-letter" are both wrong, and the correct policy is a fourth option,
"skip and record", which needs the skip to be visible in the read model rather than
swallowed. Separately, the pre-shred build denormalised the subject's name and mobile
number into a `witness_contact` row; shredding the key does not touch that row, so **the
erasure is not complete until the projection is rebuilt** — and there is no `reset`, so
an Article 17 obligation is discharged by code the port cannot see.

**`fraud-graph` (Ladybug)** — a rebuild recreates what was erased. The graph's person
node carries a name from an older build; deleting it is a statement outside the
checkpoint transaction, and a rebuild reconstructs the node from the surviving tags,
because `subject:S-88213` is still in the index on every event. **The ring signal
survives the erasure.** Tags cannot be redacted: `Event`'s fields are private,
`with_tags` constructs a new value (`event.rs:209-221`), and there is no store-side
update path. This is the strongest argument in the catalogue for a *redaction* seam
rather than a *delete* seam, because crypto-shredding covers `data` and cannot cover
`tags` — and `tags` is the half the port has made indexable and queryable. It is not a
ledger row anywhere.

**`regulatory-extract` (one-shot HTTP)** — a backfill from position 1 in 2026, for a
store that has never run, over a log that is now partly missing below the purge line. It
must be told that, and there is no way to tell it. The auditor's finding writes itself:
the system cannot reproduce a figure it previously certified and cannot explain the
difference.

**`retention-ledger`** — a changed `Query`, mid-life. In 2024 the hold vocabulary gained
two types; adding them to the projection's `Query` means its checkpoint is now
meaningless, because every event of the new types below that position was never applied
and never will be. ADR-0007 pins a projection's subscription to `Query` deliberately
(ADR-0007:85-90); **this is the cost, and nothing in the port ties a checkpoint to the
query that produced it and nothing detects the change.** The projection silently
under-reports held claims for two years, and it is the projection the retention job
depends on.

**`subject-index`** — the projection that must not be derivable. It is itself personal
data, it must be erasable, and rebuilding it recreates what was erased. Six weeks after a
shred the SQLite file is corrupted by a power loss and rebuilt from the log; the subject
tag is still in every tag index on the hub, on the claim's Durable Object and on two
engineer devices, so the row comes back. **The erasure was executed, recorded, and undone
by a routine recovery, and nothing anywhere noticed.** A projection is supposed to be a
disposable derivation; this one is a compliance artefact.

### Sync topology

Devices to claim-DO is strictly hub-and-spoke. Claim-DO to Postgres is peer-to-peer *in
the port's sense*, because the traffic is bidirectional with different event classes each
way: claim facts originate at the edge and flow up, while erasure, legal hold and
retention originate centrally and must flow down. Neither side is only a source, and a
design that assumed one direction would have no way to tell a Durable Object that a hold
has been placed — the one message that most needs to arrive.

What this domain needs from ingest is a **third thing that does not exist**: the ability
to distinguish "nothing matched" from "nothing matched because history below *P* is not
retained", and to refuse rather than accept when the condition's coverage is not
established. That is the same retained-history primitive the retention section arrives at
independently, reached from the opposite direction. **Two unrelated requirements
converging on one missing thing is the strongest evidence available that it belongs in
the port and not in an adapter.**

### The moment it goes wrong

Monday 09:22: a claim is purged from Postgres. Sixty-three events deleted, a marker
appended. Nothing propagates to the claim's Durable Object, which still holds all
sixty-three — **the sync port forwards events, and a deletion is not an event.** An
engineer's device, last synced Saturday, also holds all sixty-three.

09:31: compliance starts placing legal holds for a police operation, working
alphabetically. Nine of the thirty-one claims fall in the purge cohort. She has not
reached the purged one.

09:33: the batch reaches a claim she *has* held. It reads, decides to purge, appends the
marker conditioned on no hold event after its read position, and is **rejected**. DCB
does exactly what it exists to do, at exactly the granularity it promises. That claim
survives; the one purged eleven minutes earlier does not.

Tuesday 14:40, offline in a steel shed: the engineer inspects a vehicle, assigns salvage
category, adds a note — three events at his local positions, each conditioned on a local
read showing a closed claim with no hold.

18:02: signal returns. The claim's Durable Object accepts at its positions 64–66; its own
history is intact, so the condition is satisfiable there and is satisfied. The DO
replicates to Postgres, where the sixty-three causal predecessors **do not exist**.
Ingest re-evaluates the condition query against the hub log and it matches nothing.

It matches nothing because there is nothing left. Ingest cannot tell that from a claim
that was never closed and never held, and no adapter could, because the port has no way
to say "history below here is not retained". The three events land.

Postgres now holds a claim file consisting of three engineer notes: no registration, no
policy, no reserve, no closure, and an open salvage instruction. On Thursday the salvage
desk works that row and releases a vehicle that is police evidence.

*One correction to the ending.* The Thursday chain is decoration. The load-bearing failure
is at 18:02. That the desktop projection then materialises a header row from three notes
with no `ClaimRegistered` presumes a projection that upserts rather than one that fails on
a missing parent — a projection-design choice, not a port property.

### What the scenario walked past

Its best original element — hundreds of thousands of independent `SequencePosition` spaces
merging into one — is exercised in exactly one direction. **Nothing tests what a *device*
does when the hub rejects its ingest**, which is the case where the device holds events it
believes are committed and the hub disagrees. No other scenario in this catalogue covers
it either. That is where the sync port's shape is genuinely undetermined.

---

## 5 — Turnstile — a stadium onsale run entirely at the edge

### The domain

Turnstile runs primary onsales for stadium tours: 14 dates, 880,000 tickets, the largest
being 62,850 seats across 412 seating blocks. 1.4 million fans are pre-seeded into a queue
overnight and drained over 40 minutes, with 380,000 concurrent inside the first four
seconds. Deployment is entirely Cloudflare: one Worker per request across 310 points of
presence, one SQLite-backed Durable Object per (date, block) holding that block's event
log, and a Neon Postgres branch reached over the one-shot `/sql` endpoint holding the
tour-wide shared log where the per-fan entitlement cap lives. There is no long-lived
process anywhere: no connection, no interactive transaction, no cursor, no thread, and no
guarantee that the caller is still alive when a write lands. On show night, box-office
terminals and handheld scanners at the venue run the same stack over local SQLite and go
offline for the duration of the support act.

*Arithmetic health warning.* 62,850 seats across 412 blocks is 152 seats a block, but
`BlockOpened` carries `seat_count: 1140` and the availability read model is "exactly 1,140
rows". Every per-block number in the scenario is wrong by 7.5× in one direction or the
other, which matters because the cold-start fold cost, the log size and the alarm batch
size are all derived from it. **Read the block as ~152 seats**, and read the hold-expiry
alarm batch as ~12 rather than 400 — 400 expiring holds per block is more holds than the
block has seats, nearly three times over even at the larger figure. A batch of 12 makes
the same argument and is defensible.

### The axis

**Nothing can be held open.** Every other shape in the workspace assumes a process that can
hold a connection, a transaction and a cursor open for the duration of one decision, and
that the caller who started an append is still there when it finishes. Here the store is
reached over one round trip with no session, the "stream" is a sequence of independent
statements, and the caller vanishing mid-append is the *normal* case.

*The correction, and it halves the claim.* The Durable Object is **not** at that far end,
and the scenario says so itself: the DO's input gate does what `MemoryEventStore` does with
an `RwLock` (`memory.rs:189-195`). A DO serialises its writers and assigns positions under
an implicit lock, which makes it the **fifth adapter of the same storage shape** CLAUDE.md
warns about. Only the Neon half sits at the other end, and Neon is already the workspace's
designated instrument for exactly that axis. The scenario overclaims for half its
deployment.

What survives, and is genuinely at the far end, is narrower and still valuable:
`AppendError`'s three-way taxonomy examined by a caller who does not know whether its write
landed, and an append condition examined on a store that does not serialise its writers and
cannot take a predicate lock.

### Vocabulary

`BlockOpened` `{tour, date, block}`, `SeatBanded` `{block, seat, band}`, `HoldPlaced`
`{block, seat, fan, hold}`, `HoldExpired`, `HoldReleased`, `SeatTicketed`
`{block, seat, fan, order, hold}`, `SeatReleased`, `SeatTransferred`, `BlockSuspended` /
`Resumed`, `SeatMapRevised` (340 KB of GeoJSON in `Event::data`), `PriceBandRemapped`.
Shared log: `QueueAdmitted` `{tour, fan}`, `AllocationReserved` / `Settled` / `Released`
`{tour, fan, order}`, `FanBlocked`. Venue: `DoorSaleRecorded` `{block, seat, terminal,
order}`.

**The hold id is a tag, not a payload field**, and that is not decoration. It is the only
thing a retrying caller can query on, because `Event::data` is opaque
(`event.rs:159-166`) and `QueryItem::matches` filters on type and tags only
(`query.rs:113-116`). The recovery path after an unknown outcome therefore forces
client-generated identity into the tag vocabulary — **a design constraint the contract
creates and never states.** This generalises, and the scenario did not generalise it: any
identity a recovering or deduplicating caller must address has to be a tag.

### Decisions

**D1 — PlaceHold.** Up to four contiguous seats plus the block's suspension state: one item
per seat over five types, plus one item on `{block}`.

`AppendCondition::new(query).after_opt(last_seen)`, **deliberately self-matching**:
`HoldPlaced` is in the condition's own type list and carries the seat tag, so a retry after
an unknown outcome is structurally rejected rather than duplicated. **This is the only
idempotency mechanism in the whole design** — no dedup table, no idempotency key — and it
falls straight out of DCB.

It also composes end-to-end on a `!Send` runtime, and this was compiled rather than
reasoned about: a `RefCell`-backed `EventStore`, a generic
`place_hold<S: EventStore>(store: &Rc<S>, ..)` that builds five items, calls
`store.read(&query, ReadOptions::new())`, drains it through `collect`, builds the condition
and calls `store.append` — **no `Send` bound anywhere on that path.** `read` returning
`impl Stream` at the top level of the return type (`store.rs:117-121`) is exactly what makes
it work, and ADR-0001's insistence on that is load-bearing. This confirms the two-flavour
design from the wasm side for the first time; every previous justification was a
`cargo check`.

Because `append` takes `&self` (`store.rs:141-145`), twelve concurrent requests inside one
Durable Object share one store through an `Rc` with no `&mut` and no lock of the adapter's
own. Which raises the question the port does not answer: **may a second `append` be entered
while a first is in flight on the same `&self`?** On the `!Send` flavour both futures
interleave at every `.await` on a single-threaded executor. A `RefCell`-backed adapter that
holds its borrow across an awaited storage call panics at runtime; one that drops the borrow
has an unspecified window between the condition probe and the write. `MemoryEventStore`
cannot surface this: `memory.rs:184-224` contains no `.await` at all, so its lock is never
held across a suspension point.

*Two of the scenario's claims here are false against the code.* `type Error:
core::error::Error + 'static` (`store.rs:99`) carries **no** `Send` or `Sync` bound, so an
adapter error holding a `JsValue` or an `Rc<str>` compiles today — the stringification cost
the scenario attributes to the current contract is the cost of a *proposed* change to add
`Send + Sync`. And the monomorphisation-against-a-10 MB-bundle argument is overstated: a
test-only memory store does not ship, so it is two copies of a read-fold-append loop. The
genuine consequence of no `dyn EventStore` (ADR-0001:88-90) is **runtime store selection** —
local SQLite when offline, the DO when online — which the scenario never raises and which
is the first thing an application will ask for.

**D2 — ConvertHoldToTickets.** The seats leg of a purchase, after the card is authorised.
This is the append that gets cancelled: the Worker is holding the fan's HTTP connection
while it fetches the DO, the fan's connection drops, Cloudflare cancels the Worker, and the
`append` future is dropped mid-flight. The DO has already received the request; the row is
durable, and nobody is left to be told.

The contract's only atomicity statement is `store.rs:130-133` — *"Either every event lands or
none does. A rejected append must leave the store byte-identical"* — which is about **partial
batches** and says nothing about a dropped future. None of the 27 rules polls a future once
and drops it. `AppendError` needs no new variant for this, because a dropped future returns
nothing at all; what it needs is one paragraph stating whether an adapter may commit after
its future is dropped. `MemoryEventStore` passes any such rule trivially, which is exactly
why the reference store cannot answer the question and a real adapter must.

**D3 — ReserveTourAllocation.** The entitlement leg, on Neon over one round trip. This is
the invariant that cannot live in any Durable Object because it spans 4,912 of them.

It fits in one statement, and the statement is worth recording because it **corrects a
finding already in the repository**: a `WITH cond AS (…), ins AS (INSERT … SELECT … WHERE NOT
EXISTS …) SELECT …` shape does the condition check and the write in one implicit transaction
*and yields `conflicting_position`* — which `docs/evaluation/review-adapter-implementability.md`
concluded that shape could not produce. That argues for `conflicting_position` staying a hint
(`error.rs:96-104`) rather than becoming a promise, since the shape thought to preclude it can
supply it after all.

What it does not survive is concurrency. `nextval()` allocates outside the transaction and
`NOT EXISTS` takes no predicate lock, so under READ COMMITTED two statements for the same fan
neither see nor block each other — they insert different rows — and both succeed. The fix that
stays inside one round trip is an advisory lock at the head of the CTE, and the only key an
adapter can compute without domain knowledge is the condition's **tags**, taken in sorted
order. That is exactly correct under DCB's superset semantics: any event that could violate
condition *C* carries every tag of some item of *C*.

It has one degenerate case that must be said out loud. **A type-only condition has no lock key
and collapses to a global lock over the shared log** — and a type-only condition is legal,
idiomatic and encouraged: `QueryItem::of_types` is a first-class constructor
(`query.rs:85-91`), the append documentation's own worked example uses it (`append.rs:23-31`),
and only a *fully* unconstrained item is rejected (`error.rs:66-71`). One naive command handler
makes the entire onsale single-threaded.

**D4 — ExpireHolds.** A DO alarm expires holds past their deadline. Its condition is
deliberately **not** self-matching — `HoldExpired` is absent from its own type list — because
expiring a batch and retrying after an unknown outcome would otherwise be rejected wholesale
even when only the first few landed. So it double-writes silently under retry, and there is no
caller to observe the failure: an alarm handler has no client. The alarm is also the most
cancellable thing in the system — a DO is evicted for inactivity, restarted on a runtime
update, or reset by an uncaught exception in a sibling handler — and its future is dropped with
no notification anywhere.

**D5 — SellAtDoor.** A box-office terminal sells a door-allocation seat while the venue uplink
is down. The condition is sound locally and vacuous globally.

`AppendCondition` holds one `after: Option<SequencePosition>` and a `Query`
(`append.rs:52-61`). The terminal knew **two** things when it decided — its own local position,
and the highest hub position it had replicated before the uplink dropped — and only the first is
representable. Re-checking at the hub with `after: None` rejects the terminal's own
already-replicated events; re-checking with a translated position is impossible because
`SequencePosition` is store-local by construction.

`AppendCondition` is `#[non_exhaustive]` with *public* fields (`append.rs:51-61`), so an added
field is a minor version bump — but the honest fix is probably an ingest-side condition type in
`happenstance-sync` carrying `(origin_peer, origin_position, local_after)`, because the hub's
re-check is a different operation from a local append and putting it on `AppendCondition` would
put replication semantics in the contract crate.

### Projections

**`block-availability` (SQLite inside the same DO as the event log)** — the only projection
store co-located with its event store, and therefore the only one where the checkpoint
invariant is free. Cold start after eviction must resume from the checkpoint, which is where
`ReadOptions::from` being *inclusive* bites: `from(checkpoint)` re-applies one event and
`from(checkpoint.next())` relies on a method whose own documentation disclaims it for gapped
stores (`event.rs:138-144`). The idiom is in fact sound — a forward read from a position no
event occupies skips nothing — but nothing says so, `next()` returns `Option` (forcing an
overflow branch in library code that may not `unwrap`), and every adapter author will re-derive
the argument.

`PriceBandRemapped` on onsale morning moves seats between bands, which changes what the fold
*means* for every prior `SeatBanded`. Rebuild from position 1 — and `store.commit(batch, id,
None)` is `error[E0308]: expected SequencePosition, found Option<_>`.

**And the finding that fixes a claim the catalogue elsewhere overstates:** a `Batch` that is a
buffer of pending statements replayed inside the DO's callback-scoped transaction **satisfies
`ProjectionStore` without contortion**, and a generic pump taking
`F: FnMut(&mut P::Batch<'_>, ..)` compiles and writes. The unbounded GAT does not bite this
scenario at all. Only code that must write to a `Batch` it knows nothing about is stuck, and
nothing here needs that. What *is* lost is read-your-own-uncommitted-writes within a batch,
which makes a projection needing it unwritable at the edge.

**`tour-fan-ledger` (Neon over one-shot HTTP)** — a projection store on the far side of a
network with no session. A backfill has no cursor, so the read is a sequence of independent
paginated statements — and **chunked reads are not a snapshot.** Each statement is its own
transaction, so the checkpoint recorded at the end of one chunk may be *below* the position of
an event that became visible after that chunk was read, because `nextval()` allocated it
earlier and committed it later. Those events are skipped permanently, with no error and no
failing test.

That is the same invariant as axis 2 seen from the projection side, and it is why the
statement belongs on `read` and covers both: **the contract never says whether a `read`'s
result set is fixed at first poll or may grow**, and `MemoryEventStore` (which filters under
the lock and streams from a snapshot, `memory.rs:157-181`) and a self-paginating adapter are
both conformant with opposite semantics.

**`seat-neighbourhood` (Ladybug)** — a poison event, and the per-`(store, ProjectionId)`
checkpoint means it halting does not hold up the other three, which is ADR-0007 working.

**`block-heatmap` (in-memory in the DO)** — reset on every cold start by construction, and the
only projection for which reading the whole block log is *correct*. The contract offers nothing
to shorten it; the closest thing to a `head()` is `read(&Query::all(),
ReadOptions::new().backwards().limit(1))`, which is one cheap statement on DO SQLite and one
full HTTP round trip on the adapter with the smallest latency budget in the system. At the
edge, **`ProjectionStore` *is* the snapshot mechanism**, and this is the case that has to do
without it.

### Sync topology

Hub-and-spoke for the block DOs replicating a filtered slice up to the shared log, one
direction only. Peer-to-peer on show night, because the uplink dies and the terminals must
keep selling.

*Dropped.* Thirty handheld scanners syncing peer-to-peer over the venue LAN is asserted in the
topology and never walked; only one terminal decision appears, and it produces the same finding
a two-peer story would. Either a scanner decision that conflicts with a terminal decision
belongs in the decisions list, or the scanners come out. They come out.

**Peer D is the one that earns its place**: a legacy KV-backed Durable Object whose value cap is
128 KiB. The contract bounds the two fields no engine struggles with — `MAX_EVENT_TYPE_LEN` and
`MAX_TAG_LEN`, both 255 (`event.rs:14`, `tag.rs:14`) — and leaves `Event::data` unbounded. A
340 KB seat map is durable at origin and structurally unrepresentable at that peer, and the
incompatibility is discovered at ingest, **after the write has already committed somewhere
else.** There is no trait in `happenstance-sync` at all (`sync/lib.rs:103-112`), so there is
nowhere to declare a peer capability.

*A structural criticism of the deployment, worth recording.* One Durable Object per (date,
block) with tag spaces disjoint by construction is stream-per-aggregate wearing a DCB API. Every
command in the block draws a boundary that is a subset of one object's log — which is precisely
the pre-drawn boundary DCB exists to dissolve. Only `PlaceHold` genuinely earns the dynamic
boundary, because four seat items plus the block's suspension state is a real multi-entity
invariant. And the tour-wide cap is presented as "DCB's cross-aggregate case in its purest
form" while being implemented as a **saga** — reserve, then the seats leg, then settle or
release, across two stores with compensation. DCB's claim is that this saga is unnecessary.
Reinstating it and calling it the purest case inverts the argument. It is an honest deployment
consequence — no consistency boundary can span 4,912 Durable Objects and a Neon branch — but it
demonstrates the **limit** of DCB's claim, not the claim.

### The moment it goes wrong

At 10:00:04 the first thing goes wrong and the design catches it. A fan on a degrading
connection has authorised a card; the Worker has written the allocation to the shared log and is
mid-flight on the DO fetch that will write `SeatTicketed`. The connection drops, the Worker is
cancelled, the append future is dropped. The row is durable and nobody is left to be told. At
10:00:31 the fan retries; the new Worker re-reads the seat query, finds `SeatTicketed` tagged
with **its own hold id**, concludes the first attempt landed, and settles. Recovery works — and
it works for exactly one reason: the hold id is a tag.

At 10:07:12 the second thing goes wrong and nothing catches it. A fan is queuing on two devices,
which every fan does. Both reach the front within the same millisecond. Two Workers, two points
of presence, two one-shot statements against Neon, each carrying the fan-cap condition. One takes
position 118,204 and spends 26 ms probing; the other takes 118,205, probes in 1.2 ms, and
commits first. The first statement was already inside its own snapshot when that happened, so it
sees nothing above its boundary either, and commits at a *lower* position. Neither is wrong about
anything it could see. Under READ COMMITTED a `NOT EXISTS` takes no predicate lock, and the two
rows do not conflict because they are different rows.

The fan holds four reserved from two commands that each believed itself alone. A third device
would make it six.

Nothing alerts. The shared log is internally consistent; every event has a unique, monotonic
position; every append condition was honestly evaluated. It surfaces the following Monday, when
finance reconciles against the card ledger and finds 1,847 fans over a cap of four — 31 of them
at eight tickets, which are the bot farms, because they queued on more devices than anyone else
and therefore harvested the bug most efficiently.

The Neon adapter passes all 27 conformance rules. It passes
`racing_conditional_appends_elect_one_winner`, **because that rule is written sequentially on
purpose**: a genuinely parallel version would need `Send + Sync + 'static` bounds `EventStore`
deliberately does not carry, and the rule's own doc comment says so (`suite.rs:589-595`). Every
other adapter in the workspace serialises its writers and satisfies the rule for free. The suite
has never been shown to reject a store that gets append conditions wrong under real concurrency,
because until this deployment no such store existed to reject.

---

## 6 — Kestrel Rotor — offshore wind spares between a depot and a vessel

### The domain

Kestrel Rotor Services runs spares logistics for offshore wind operations and maintenance:
22,400 SKUs across two shore depots and whatever is aboard a vessel, of which 3,100 are
serialised — rotating equipment, HV components, certificated lifting gear. Two depots run
Postgres behind a permanent WAN link. A service operations vessel runs SQLite on a rugged server
in the ship's office and is itself a hub for nine technician tablets over ship's wifi. Crew
transfer vessels carry tablets reaching a per-windfarm Durable Object over cellular. A Neon
reporting replica ingests over one-shot HTTP for finance. The vessel reaches shore only through
one depot — the satellite plan terminates at a single teleport and that depot is the sole
whitelisted endpoint — giving **one usable sync window a day, median 34 minutes**, taken when the
vessel is stationary and the crane is not blanking the antenna.

Every one of these nodes both reads and writes. The vessel consumes parts mid-repair 40 nautical
miles offshore and cannot wait for a quorum; the depot sells and ships from the same stock while
it does.

*Arithmetic health warning.* 1,100–1,800 events a day over three years is 1.2M–2.0M events, not
the 4.2M claimed; at 4.2M the implied rate is roughly double the stated range before campaign
spikes. The rebuild timings hang off the 4.2M figure, and the claimed 40-second Postgres rebuild
over it is about 105,000 row-at-a-time upserts a second, an order of magnitude optimistic. The
shape is what matters; the numbers are not load-bearing and are not relied on here.

### The axis

**The number of independently authoritative writers to one logical fact set**, and everything
else in the workspace sits at exactly one. `MemoryEventStore`, a `RefCell` store, rusqlite, a
Durable Object, Postgres and Neon differ in how they assign positions and how they hold a
transaction — but each is the sole authority for its own log, so an `AppendCondition` evaluated
against it is evaluated against the complete set of facts.

This sits at *N*, with *N* unbounded and its membership changing as vessels charter in and out, and
with the incompleteness of any one log being the **steady state rather than a fault**. It is the
only shape in which two fully conformant stores, both passing all 27 rules, jointly produce a
wrong answer — the failure has no within-one-store expression at all.

*One framing correction.* The scenario says the closest existing rule,
`racing_conditional_appends_elect_one_winner`, "is precisely the rule that must NOT hold across
the boundary". That presents an absence as a contradiction. The rule is written against one store
from one factory (`suite.rs:596-638`) and makes no cross-store claim; there is no cross-store rule
to be in tension with. The honest statement is that **the suite has nothing to say about two
stores**, which is a gap rather than a conflict.

### Vocabulary

`PartReceived` `{part, serial, lot, location, grn}` — and a bulk variant with no serial tag and a
quantity in the payload. `SerialisedUnitAllocated` `{serial, job, location}`,
`SerialisedUnitAllocationReleased`, `SerialisedUnitConsumed` `{serial, job, turbine, farm}`,
`UnitScrapped`, `UnitQuarantinedForDefect`. `UnitTransferInitiated` `{serial, movement, from,
to}` / `UnitTransferReceived` / `UnitTransferCancelled`. `BulkStockIssued` `{part, location,
job}`, `StockCountAdjusted`. `CertificationExamined` / `CertificationExpiryRecorded`
`{serial, cert}`. `PurchaseOrderCommitted` / `Priced`, `JobOpened` / `JobClosed`,
`AllocationConflictAdjudicated` `{conflict, serial, job}`.

### Decisions

Three, down from six. D1, D3 and D5 were three dramatisations of one breakage — that an
`AppendCondition` is a claim about a *log* rather than about the *world* — and each is well told
without adding a distinct finding beyond the first. D1 is kept.

**D1 — AllocateSerialisedUnit.** One `QueryItem`, seven types OR'd, one tag: the whole life of one
physical object.

```rust
Query::from_item(QueryItem::new(
    ["PartReceived", "SerialisedUnitAllocated", "SerialisedUnitAllocationReleased",
     "SerialisedUnitConsumed", "UnitTransferInitiated", "UnitTransferReceived", "UnitScrapped"],
    Tags::from_pairs([("serial", "PB-4412-0087")])?,
)?)?
```

The condition is `after_opt(last_seen)`, where `last_seen` is 38,102 aboard the vessel and
3,918,442 at the depot **for the same fourteen events**. There is no expression relating them.
The two conditions encode identical knowledge and are not comparable, so ingest cannot re-check
this condition even if it wanted to: it would have to invent a position that does not exist in
the receiving store.

**D2 — IssueBulkFromVesselStock.** The control case, and it is a real control case. Two tags AND'd
— `part:HYD-0119` and `location:sov-aurora` — over five types. On-hand never goes negative, and
under partition this condition is **genuinely sound**.

Its soundness is accidental as far as the library is concerned. It is partition-safe only because
the tag set contains a location key **exactly one peer ever writes**, so the local log is complete
with respect to *this query* even when it is incomplete with respect to everything else.
AND-within-an-item is what makes that work, and that semantic is conformance-pinned
(`suite.rs:96-120`, `:148-163`).

**Partition-tolerance is bought here by the tag vocabulary, not by the port.** Nothing lets a domain
declare `location:*` peer-owned, and nothing can check that a query is closed under a peer's write
set. `Tag` is an opaque `Box<str>` (`tag.rs:39`) with `key()`/`value()` as convention helpers over
`split_once(':')` (`tag.rs:88-98`), and a tag with no colon is explicitly legal. The distinction
between D1 and D2 is the single most useful thing an application designer needs to know, and it is
the least expressible thing in the API.

*Correctly scoped:* this does **not** belong in the contract crate. Putting ownership semantics into
`Tag` would make it non-opaque and exceed the DCB specification. It belongs to the sync runner as a
per-peer declaration of owned tag keys, with a check that a decision's query is closed under it.
Worth stating explicitly so the omission is a decision rather than an oversight.

**D4 — IssueLiftingAccessory. Cut as written; one finding salvaged.** The claim was that the
invariant — a certificated lifting accessory may not be issued after its examination expiry — is
falsified by the passage of time, and time is not an append, so "there is no event to condition on".
**The scenario's own event list contains `CertificationExpiryRecorded`**, which is exactly the
materialise-the-clock-as-an-event answer; with it, the condition
`{CertificationExpiryRecorded, UnitScrapped, UnitQuarantinedForDefect}` tagged `{serial}` with
`after: last_seen` enforces the invariant precisely. The premise is false in the scenario's own
vocabulary.

What survives, and is real: **the store records no time.** `SequencedEvent` is
`{ position, event }` (`event.rs:277-282`), so "which side of midnight on 30 September did this
issue fall" is unanswerable from the log, and every timestamp in the system is a writer's clock —
here, one drifted by 2.4 seconds — embedded in an opaque payload. That is an audit failure, and it
is about `SequencedEvent`, not about `AppendCondition`.

**D6 — RecordConsumption.** The bearing is torqued into the pitch system and the technician has
climbed down. There is **no invariant that may refuse**; the log's job is to record what happened,
and a store that declines is simply wrong about the world. `append(&events, None)` is expressible
and cheap (`store.rs:144`) — DCB does not force a condition on a command that may not be refused,
and neither does the contract.

What is missing is the **discriminator**. `Option<AppendCondition>` renders "I asserted nothing
because there was nothing to assert" and "I am a decision whose author forgot the condition" as the
same bytes. The reconciliation projection must distinguish them — an observation cannot lose a
conflict, a decision can — and it cannot.

### Projections

**`stock-on-hand`** — the only one of five that converges, and it converges because the fold is a sum
of signed deltas: commutative and associative, so peer arrival order is irrelevant. **It is safe
because someone chose addition, not because anything checked.**

**`unit-ledger`** — a per-serial state machine, order-dependent within a serial and order-independent
across serials. It converges only if events for one serial are applied in an order both peers agree
on, **and local position order is not that order.** This is the projection that produced the
divergence. There is no such order available: `SequencedEvent` carries only the peer-local position,
and ADR-0007's runner hands `Sequenced<Self::Event>` — the same peer-local value (ADR-0007:76-81).

**`cost-layers`** — deliberately and permanently non-convergent. FIFO consumption order *is* the fold,
so the answer is a function of the order events are applied in, and no order exists that two peers can
agree on. It is pinned to one depot's arrival order and declared authoritative by fiat. A rebuild does
not reproduce signed-off figures once a late-arriving vessel issue has been interleaved, so **the
projection must refuse to rebuild before the last locked period** — and nothing in `ProjectionStore`
can express "cannot be rebuilt before position *P*", and nothing lets a projection declare itself
non-convergent so that a convergence check knows to skip it.

**`conflict-queue`** — the projection whose whole job is to see conditions, and the one the port cannot
subscribe. **No append condition is persisted by anything, anywhere.** `append` takes it as a
parameter, evaluates it, and drops it (`store.rs:141-145`); `Event` has four fields and none of them
is a condition (`event.rs:183-188`). The scenario says the condition is "envelope rather than event",
which understates it — **there is no envelope either.** The only place to put it is
`Event::metadata`, an `Option<Bytes>` the writer fills in by hand, which means the store never checks
that the recorded evidence matches the condition it actually evaluated. The projection would be
reasoning about an unattested self-report. And even that cannot be subscribed selectively: `Query`
matches on type and tags only, so the projection must subscribe to `Query::all()` and decode
everything.

*One correction.* The scenario demands an HLC for this projection and supplies its own refutation. Its
stated requirement is byte-identical output on every peer, which needs a deterministic
**peer-independent total order** — and sorting by the `(origin, origin_position)` pair the RUNBOOK has
already decided delivers exactly that. It conflates determinism with causality. Only the second needs
an HLC, and no stated requirement needs the second.

**`unit-provenance` (Ladybug)** — retained for exactly one observation, which is worth the space. A
replicated `SerialisedUnitConsumed` carried `turbine:HW2-A14 ` with a **trailing space**, which
`Tag::new` accepts: it rejects empty, over-length and control characters, and U+0020 is none of those
(`tag.rs:48-59`). `Tags` sorts canonically, so the variant is a distinct tag sorting adjacent to the
real one, and `contains_all` — a strict merge-scan on equality (`tag.rs:231-245`) — does not match it.
The recall query for a lot advisory silently misses a turbine. The fix is not in `Tag::new`: the
specification says tags are opaque strings and trimming would silently rewrite a caller's data. It is a
lint, a validating constructor in the typed layer, or a documented normalisation policy — plus a note
on `Tag` that **whitespace is significant**, because everyone will assume it is not.

### Sync topology, and the design it settles

Both simultaneously, because **hub-ness turns out to be a property of an edge rather than of a node**:
the vessel is a hub to nine tablets over ship's wifi and a symmetric peer to a depot over satellite, at
the same time.

*Correction.* The scenario presents this as an argument against modelling "am I the hub" as a
constructor argument. That attacks a design nobody proposed: `happenstance-sync`'s own prose already
scopes the port to a **single peer** and puts fan-out, ordering and disagreement policy on the runner
(`sync/lib.rs:28-36`). The requirement is confirmation, not discovery.

What the scenario genuinely settles, and this is its most valuable contribution, is the ingest question
(RUNBOOK:78) — with a **general** argument rather than a domain-specific one:

> **Ingest must be unconditional.** Rejection is a function of local state, so different peers reject
> different events and the union of facts is never reached. Convergence dies the moment ingest is
> allowed an opinion.

Quarantine fails for the same reason plus a worse one: a quarantined event has no local position, so it
is never forwarded, and in an A—B—C topology it disappears permanently from a third peer's view. If
quarantine is fixed by making it an appended fact, it has become unconditional append with extra
machinery and a misleading name. Unconditional-append-plus-compensation fails on emission: every peer
that ingests would emit its own compensation, and guarding emission with a conditional append elects one
winner only among peers currently connected.

Three things fall out, and they are the design:

**First, ingest appends at the tail and never interleaves.** `ProjectionStore::checkpoint` is a single
scalar that `ReadOptions::from` resumes at (`projection.rs:86-88`), so an event inserted *below* an
existing checkpoint is skipped silently and forever. There is no third option, and the port never chose
between them.

**Second, because ingest appends at the tail, local position is arrival order and two peers have
different total orders over the same events.** That is acceptable for the store and unacceptable for a
fold that must agree. So folds that must converge must be **commutative**, and the library must make
that *checkable* rather than hoped for: a sync-testkit rule that takes a projection and a set of
events, applies them under two interleavings agreeing on per-origin order and disagreeing everywhere
else, and asserts the read models are byte-identical. The wrong implementation it rejects is not
hypothetical — it is `unit-ledger`, and more generally any projection that reads
`SequencedEvent::position`. Which points at the library change: **a convergent projection should be
handed an `EventId`, not a position**, and a projection that needs position should have to say so and be
excluded from the convergence check. `cost-layers` is the honest case that needs exactly that exclusion.

**Third, forwarding must be of everything a peer *holds*, not everything it *originated*.** That is what
makes the transitive step work in the vessel—depot—depot topology, and it works precisely because arrival
order and forwarding order are the same order. Two counterexamples, both real: a quarantining peer breaks
it (above), and **a peer that compacts its log under a retention policy while another peer is offline
longer than that window can never converge, and nothing anywhere reports that it did not.** Retention has
to be coordinated across the peer set, and no port in the workspace has a place to say so.

### The one thing the port cannot pay for

The sharpest finding in this scenario, and the one nothing else in the catalogue would have found:

**`append` takes `events: &[Event]` and a single `Option<&AppendCondition>`**, so one consistency
boundary covers the whole batch and the batch is all-or-nothing (`store.rs:141-145`, atomicity at
`:130-133`, pinned by `suite.rs:385-403`). Idempotent bulk ingest needs **one boundary per event**. The
three options are all wrong:

- One `append` per event: 1,840 round trips, fatal on the one-shot HTTP peer in a 34-minute satellite
  window.
- A `Query` of 1,840 single-identity items: legal, but one already-seen event rejects the other 1,839,
  and `ConditionViolated.conflicting_position` is an `Option` naming at most one culprit
  (`error.rs:96-104`), so recovery is a serial peel.
- Read-then-filter-then-unconditional-append: two round trips, and nothing closes the race when a depot
  ingests from two peers concurrently — the `AppendCondition` is the only mechanism that could.

*One mechanism the scenario proposes and this catalogue rejects.* Materialising event identity as a tag
(`oid:sov-aurora-3f9c#38103`) so ingest can express dedup as an ordinary `AppendCondition` is presented
as free. It is not. It overturns a **decided** ledger row (RUNBOOK:70 — `EventId` as a store-assigned
Lamport pair on `SequencedEvent`); it makes identity writer-forgeable; it enters every `contains_all`
merge-scan on every query; and because tags participate in matching, tag-only queries now see an identity
dimension the domain never asked for. It also has a hole the scenario does not notice: it is described as
carried "on every **ingested** event", so a peer's own locally-originated writes carry none and the peer
cannot order its own events against ingested ones. **The mechanism only works if every store stamps its
own writes at append time** — which is the store-assigned `EventId` the RUNBOOK already decided, not a tag
the sync layer bolts on.

What survives from it, and it is a genuine requirement: whatever `EventId` becomes, **a peer must be able
to dedupe without parsing `metadata`.** A peer that must parse opaque bytes to dedupe has broken ADR-0003
at the exact point ADR-0003 claims to win.

### The moment it goes wrong

At 06:40 a depot releases a pitch-bearing assembly from a cancelled job; it lands at depot position
3,918,442 and reaches the second depot two seconds later. At 07:12 the vessel's satellite window opens and
it ingests 1,840 events including the release, which lands at vessel position 38,102.

At 09:55 the vessel's spares coordinator plans a lift. Her decision model reads the unit-life query, folds
fourteen events, sees the unit free at the depot, and appends `SerialisedUnitAllocated` under
`after: Some(38,102)`. At 10:03 the cancelled job is reinstated; the depot controller reads the same query
against the depot's log, folds the **same fourteen events**, sees the unit free, and appends
`SerialisedUnitAllocated` under `after: Some(3,918,442)`.

Both stores were right. Both appends were conformant. `racing_conditional_appends_elect_one_winner` passes
on each of them, because it is a rule about one store and there were two.

The vessel is dark until 07:09 the next morning. Then it ingests the depot's allocation, which lands at
vessel position 38,441 — **three hundred and thirty-eight positions after its own at 38,103.** From the
vessel's total order, the depot allocated a unit that was already held. The same morning the depot ingests
the vessel's allocation at 3,919,905, after its own at 3,918,779. From the depot's total order, the vessel
allocated a unit that was already held.

**Each peer holds the identical set of facts and each names the other as the loser.** Nothing about that is
a defect in either store. It is a projection folding over `SequencePosition`, a value that means something
different on each side of the link, handed to it by the runner as part of `Sequenced<Event>`.

At 11:20 the bearing goes onto a crew transfer vessel bound for the wrong farm, because the depot's ledger
says so and the depot is where the part physically sits. The next morning a lift stands up a 400-tonne
crawler, a weather window and a jack-up day rate, and the bearing is 130 km away.

When the duty controller asks which allocation came first, **the honest answer is that nothing in the system
knows.** 38,103 and 3,919,905 are not comparable in any way that means anything. There is no store-assigned
time on a `SequencedEvent` anywhere in the workspace. The Lamport pair the runbook has settled on gives
dedup and a deterministic tiebreak, and a tiebreak is not a fact about the world. The two clocks were 2.4
seconds apart, which would have been plenty, and neither was ever read.

The failure was never that a conditional append was accepted on both sides. **Both had to be.** The failure
is that the answer to who won was computed from arrival order on a peer instead of from the merged log — and
that the one projection whose job is to notice cannot be written, because the conditions it would need to
compare are neither event nor envelope, and `Query` cannot see them.

---

## What the six agree on

Where six deliberately dissimilar deployments converge is the useful signal. They converge on nine things.

1. **`ProjectionStore` cannot reset.** All six. `commit` takes a `SequencePosition` and `checkpoint`
   returns an `Option`, so "never run" is readable and unwritable, and the closest available substitute —
   `commit(empty, id, FIRST)` — silently skips event 1.
2. **`Batch` has no write vocabulary.** All six. Generic code can `begin`, `commit` and `rollback` and
   cannot put a row in a batch, so nothing generic can be a projection.
3. **There is no event identity.** Five of six. Ingest cannot be idempotent, an outward-writing projection
   cannot avoid re-emitting, and a convergent fold has nothing peer-independent to order by.
4. **`AppendCondition::after` is store-local and serialises as a naked integer.** Five of six. The feature
   that exists specifically for replication makes the single most dangerous replication mistake the path of
   least resistance.
5. **Position order is not stated to be visibility order.** Four of six, from both the append side and the
   projection side.
6. **A read is not stated to be a snapshot**, and the items of one `Query` are not stated to share one.
   Three of six, and the two adapters that would violate it are the two the workspace has already chosen as
   instruments.
7. **`append` takes one condition per batch.** Two of six, from opposite directions: the decomposition of a
   peer's unit of work, and the impossibility of idempotent bulk ingest.
8. **A store cannot say what it does not hold.** Three of six — a pruned slice, a purged cohort, a compacted
   peer — and all three arrive at the same missing primitive from different doors.
9. **Nothing decides what happens when `apply` fails**, and two scenarios prove the policy cannot be one
   policy: halting is correct for a revenue ledger and wrong for an availability board, and a deliberate
   crypto-shred needs a fourth option that is neither retry nor dead-letter.

And on one thing they agree the design got right, which is worth as much: **the query language never needed
extending.** Thirty-odd boundaries across six domains, including several that span what would classically be
three or four aggregates, expressed without strain in `Query`, `QueryItem` and `Tags`. Types-OR within an
item, tags-AND with superset matching, items-OR across the query is exactly the shape every fold in this
catalogue needed. No scenario wanted a filter the language cannot express.

Two ADRs are corroborated at precisely the point most likely to break them. **ADR-0001**: a `!Send` command
path was compiled, not argued — `read` returning the stream at the top level is what makes it work, and the
`!Send` peer sitting mid-chain rather than at a leaf is what makes CLAUDE.md rule 4 binding on the sync
runner too. **ADR-0003**: a hub runs a domain decision, adjudicates a conflict and authors a compensation
without ever parsing a payload — because every fact it needs is in the type, the tags and the condition
query. That is a constraint the library should state out loud: **anything replication must reason about has
to be in the tags.**
