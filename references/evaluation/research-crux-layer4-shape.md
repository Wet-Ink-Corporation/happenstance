# Exploration: the shape of layer 4, worked against a real application

Status: **exploration notes**. Fourth of four. Nothing here is normative, and nothing here is an ADR.
Date: 2026-08-08.
Companions: `research-crux-integration.md`, `research-crux-composition.md`, `research-crux-stack.md`.

This document assumes the stack in `research-crux-stack.md` §1 exists exactly as postulated, and asks
what layer 4 — the application framework — actually *feels* like to build against. Architecture and
patterns only; no code.

---

## 1. The example: an independent bookshop

Small enough to hold in the head, real enough that people build it, and it has genuinely separate
domains rather than one domain cut into arbitrary pieces.

**Domains** (each a bounded set of facts, each owning a tag namespace):

| Domain | Namespace | Owns |
|---|---|---|
| Catalogue | `catalogue.v1` | titles, editions, ISBNs, prices, supplier links |
| Stock | `stock.v1` | receiving, on-hand counts, stocktakes, damage/write-off |
| Till | `till.v1` | sessions, sales, refunds, cash-up |
| Orders | `orders.v1` | special orders, customer reservations, collection notices |
| Events | `events.v1` | author readings, ticket allocation |

**Devices** — every one of them a peer store, none of them authoritative:

| Device | Role |
|---|---|
| Till A, Till B | selling; must work when the line is down, which for a shop till is not optional |
| Stockroom tablet | receiving deliveries, stocktakes, write-offs |
| Owner's phone | dashboards, approving write-offs, checking a customer order |
| Back-office laptop | catalogue maintenance, supplier ordering, events |
| A cheap always-on box | not an authority — an archival peer, per `research-crux-integration.md` §9.1 |

Two properties make this the right example. First, the invariants genuinely span domains: you cannot
sell a copy without the catalogue, the stock and an open till session all agreeing. Second — and this
is the part the micro-frontend literature has no vocabulary for — **the five devices are five
different applications composed from overlapping subsets of the same slices over the same log.**

---

## 2. The grain

One rule, and everything below is a consequence of it:

> **Split along fact boundaries, not screen boundaries. A domain is a set of facts sharing a
> namespace; a slice is one role's view of one or more domains; screens live inside slices.**

The failure grain is the one Elm warned about — *"there is a sidebar, so I need a `Sidebar` module"* —
and its modern equivalent, splitting along team boundaries before domain boundaries. Both produce
modules that are ontological arguments rather than units of reasoning.

The corollary that does most of the work: **domains are shared, slices are not.** A slice that needs
a title's price depends on the *Catalogue domain crate* (read-only projections and fact types), never
on the *Catalogue slice*. That single asymmetry is what stops the dependency graph from becoming a
mesh, and it is enforceable by the compiler because it is a `Cargo.toml` edge.

---

## 3. What a domain crate is

Crux-free. No intents, no `Model`, no `update`, no `view`. It is the model of a business area and
nothing else, which is why the same crate serves a till, a tablet, a dashboard and a server peer.

It contains, in rough order of how often you touch them:

- **A namespace.** One per domain, versioned (`stock.v1`). Every tag and every event type this crate
  mints goes through it, which is what makes two domain crates in one log incapable of silently
  widening each other's append conditions (`research-crux-composition.md` §8, gap G3).
- **Fact types**, each declaring its event type and, crucially, **its tags**. `StockReceived` is
  tagged into `stock.v1/title:…` and `stock.v1/location:…`.
- **Deciders** — one per meaningful decision, each declaring a `Convergence` posture.
- **Projections** — each declaring convergent or not, and each declaring the invalidation keys it
  owns.

The ergonomic payoff, and it is the thing that makes the whole layer feel light: **you declare a
fact's tags once, and both the decider's query and the projection's query are derived from those
declarations.** You never hand-write a `QueryItem`. This is what dcb.events already says the
mechanism should be — *"those queries wouldn't be written manually; they can be automatically
deferred from the decision model definition"* — and it is only expressible because the tags are the
sole thing the port can see (VT-3).

A domain crate is testable with no Crux, no shell, no async runtime: build a `MemoryEventStore`,
append some facts, run a decider, assert. And its convergence declarations are testable the same way,
by the merge property in `research-crux-integration.md` §6.2.

---

## 4. What a slice is

A slice is a role's view of one or more domains: `till-selling`, `stock-receiving`,
`orders-collection`, `events-boxoffice`, `owner-dashboard`. Note the naming — a slice is named for
what someone *does*, a domain for what *is*.

The developer's vocabulary inside a slice is **three verbs**, and getting this list short is most of
what layer 4 is for:

| Verb | Meaning | Returns |
|---|---|---|
| **decide** | run a decider: read the boundary, fold, decide, append under a guard, retry on conflict | a request resolving to accepted-or-rejected |
| **read** | query a maintained projection by key | the read model |
| **invalidate** | tell the shell which cache keys are now stale | nothing |

Everything else in a slice is ordinary Rust: an `Intent` enum with public variants and a
private-payload `Internal` variant (`research-crux-composition.md` §8), a `Model` holding
slice-local state, an `update` function generic over the app's effect and event types, and whatever
view logic the role needs.

The `decide` verb is the centre of gravity and the thing that would feel different from every other
framework: **a slice never appends a fact directly and never constructs a query.** It names a
decision and receives a verdict. Rejections are first-class and typed — `TitleNotStocked`,
`TillSessionClosed`, `WouldExceedCredit` — which means the UI's error states are enumerated by the
domain rather than invented by the screen.

---

## 5. Composition: five applications, one log

Each device role is an application. An application declares which slices it includes, and the
framework derives the rest.

```
apps/
  till/            slices: till-selling, orders-collection
  stockroom/       slices: stock-receiving, stock-take
  owner/           slices: owner-dashboard, stock-approvals, orders-overview
  backoffice/      slices: catalogue-admin, supplier-ordering, events-boxoffice
domains/
  catalogue/  stock/  till/  orders/  events/
slices/
  till-selling/  stock-receiving/  stock-take/  orders-collection/  …
```

From the slice list, the framework derives four things the developer never writes:

1. **The `Effect` union** — the set of operations any included slice requires, compile-checked
   through the `From<Request<Op>>` bounds. Add a slice needing an operation the app hasn't wired and
   you get a trait error naming it.
2. **The projection set** to maintain, and which are boot-critical.
3. **The invalidation key space** the shell will see.
4. **The replication filter** — and this one is more interesting than it looks. The till has no
   reason to hold `events.v1` facts. **The slice manifest is the natural driver for filtered
   replication**, which makes SY-27 (whole-log or filtered, currently open) a question with an
   obvious answer and a concrete consumer.

And the reframing that this example exists to make:

> **The unit of composition is the vertical slice.** Not the team, not the device, not the
> deployable. Teams and device roles are two *reasons* you might compose differently; neither is the
> thing being composed, and baking either into the architecture means the architecture is wrong the
> moment the reason changes.

The bookshop's five device roles are an *illustration* of that, not the argument for it. The same
mechanism serves product tiers, white-label variants, a slice shipped to five percent of installs, a
minimal composition assembled for a scenario harness, a feature disabled in one jurisdiction, or one
slice embedded in somebody else's application entirely — which is exactly what Proton demonstrated
with Lumo. **You do not need to know the driver in advance**, and that is the property worth having.

Three things make the slice the right unit rather than an arbitrary one. It is a **complete
vertical** — intent through decision, fact, projection, view — so it has no partner it must be paired
with. It couples to other slices **only through the log**, so including or excluding it ripples
nowhere. And it is **cheap to wire**: one `From` impl, one `Model` field, one match arm
(`research-crux-composition.md` §6).

Which mostly dissolves the micro-frontend question rather than answering it. Micro-frontends are a
workaround for architectures where composition is expensive; splitting the *deployable* was the only
lever available, so it became the vocabulary. Here composition costs a `Cargo.toml` line, so most of
what people reach for micro-frontends to achieve is available without splitting anything. What
remains — genuinely independent deployment lifecycles — is a much smaller residue, is a decision
about separate **Cores** rather than about slices, and still faces the organisational gate in
`research-crux-composition.md` §10 unchanged.

The acceptance criterion, and it is falsifiable: **can you delete any single slice and still ship the
rest?** If yes for every slice, composition is real regardless of why anyone would want to. If no for
any slice, you have a horizontal layer wearing a slice's name.

---

## 6. A cross-domain decision, worked

Selling a copy. The decision needs three things that live in three domains:

- the title exists in the catalogue — an **existence** question
- on-hand count at this shop is greater than zero — a **counting** question
- there is an open till session — an **existence** question

In an aggregate world this is a saga, or three aggregates crushed into one. Here it is a **composed
decider**: a tuple whose query concatenates three namespaces' items, whose state is the product of
three folds, and whose guard is the same query it read with. One append, one condition, no
coordinator.

The fact it emits, `SaleLineRecorded`, is tagged into **all three namespaces at once** —
`catalogue.v1/title:…`, `stock.v1/title:…`, `till.v1/session:…`. That is DCB's actual move, and it is
what lets the stock projection and the till projection both see one fact without either knowing the
other exists.

Two consequences worth internalising because they shape how the app feels:

- **The blast radius of the guard is visible in the composition.** Composing three deciders unions
  three boundaries, so conflict probability unions too. You can see, at the call site, that selling
  contends with receiving on the same title. That visibility is the feature — it is the thing
  aggregates hide.
- **The rejection vocabulary is the union of three domains' rejections**, which is exactly what the
  cashier needs to be told.

---

## 7. Offline, worked — and why the uncomfortable case is the realistic one

Two tills, both off the network, one copy of a title on hand. Both sell it.

By the taxonomy in `research-crux-integration.md` §6.1 this is **class 2, counting** — it does not
converge, and no amount of cleverness makes it converge. The available answers are a lease, escrow,
or compensation. For a bookshop:

- **Leasing stock buckets** is absurd for a shop that holds one copy of most titles.
- **Compensation is what the business already does.** Both sales complete. On merge, the stock
  projection notices negative on-hand and the domain emits `OversellDetected`. Someone apologises,
  reorders, or refunds.

That is the point worth taking away from the whole exercise: **the architecture's "unsafe" class maps
onto a business process the shop already has.** You are not inventing a compensation to satisfy the
framework; you are modelling one that exists. The framework's contribution is that the compensation
is *declared* (`Convergence::Compensable`), *tested* (the merge property fails a `Convergent` claim
that is false), and *visible in the log* rather than being an oral tradition among staff.

Contrast the cases that are easy, so the taxonomy earns its keep rather than sounding like a tax:

| Decision | Class | Offline |
|---|---|---|
| Reserve a title for a customer | idempotent existence | safe — reserving twice is reserving once |
| Record a stocktake count | authored, per-device | safe — it is an observation, not a claim about others |
| Sell the last copy | counting | **compensable**, as above |
| Allocate ticket 40 of 40 | counting, but one seller | **leased** — the box office holds a lease on that event; cheap because it is one device |
| Issue the next invoice number | uniqueness | **partition the namespace** per device, or do it online |

Four of five are fine. The design work is in naming which is which, once, in the domain crate.

---

## 8. What the shell sees

Per `research-crux-integration.md` §5.7, the read path is invalidation plus direct query, not a
pushed view model. So the shell's contract with the core is small:

- **dispatch an intent** — one method, bytes in
- **receive invalidation keys** — the effect that replaces `Render`
- **query the projection surface** — by key, either directly against the projection store or through
  a `query(key)` method on the FFI wrapper

Which means the shell is a perfectly ordinary native application with a perfectly ordinary client
cache — TanStack Query, or a `StateFlow` per key, or `@Observable` per key. The framework does not
ask the shell to learn a change protocol, hold a mirrored view model, or apply patches. That is a
large ergonomic win and it is the direct consequence of the projection store having already done the
folding.

The lifecycle the framework owns, so no application writes it twice: load the durable `StoreId`
(never mint it — `research-crux-integration.md` G1), open the store, bring boot-critical projections
to their checkpoints, start the sync runner, release the first render. The app declares only *which*
projections are boot-critical.

---

## 9. What you write versus what you get

The honest split, because a framework earns its place by this table and nothing else:

| You write | The framework gives you |
|---|---|
| facts, and their tags | tag/event-type minting through a namespace; codec registration |
| deciders: query, fold, decide, convergence | the command loop — read, fold, decide, append under guard, retry on conflict |
| projections: fold, keys | the projection runner, checkpointing, invalidation emission, rebuild |
| intents, slice models, slice `update`, view | dispatch, effect union derivation, boot lifecycle |
| which slices an app includes | `Effect` union, projection set, invalidation key space, replication filter |
| domain invariants as assertions | the N-core mesh harness, partition injection, convergence property tests, restart fuzzing |
| — | the sync runner, watermark bookkeeping, resume tokens |

The left column is the product. The right column is what currently gets rebuilt per project, badly,
and is the reason layer 4 should exist at all.

---

## 10. Where the grain fights back

Four places this design is uncomfortable, stated because a shape document that only describes the
comfortable cases is marketing.

**Boot ordering across many slices.** The owner's app has three slices and a dashboard that spans
all of them. Which projections must be caught up before first render, and what does the UI show while
they are not? Crux's app-lifecycle pattern (`Model` as an enum: `Initializing | Active`) is the known
answer and it is the one place nesting may genuinely earn its keep
(`research-crux-composition.md`, open question 4). Unresolved.

**A slice that wants a sibling slice's transient state.** The till wants to know whether the
stockroom tablet is mid-stocktake — which is not a fact, it is someone's UI state on another device.
The honest answers are: make it a fact (`StocktakeStarted` genuinely is one), or accept that you
cannot see it. There is no third answer, and pretending otherwise is how the log stops being the
integration substrate.

**Namespace version bumps.** `stock.v1` → `stock.v2` changes every tag the domain mints, which
invalidates every projection checkpoint (PS-25) and means old facts carry old tags forever. Facts are
permanent, so a namespace version is a permanent fork in the tag space. This needs a designed answer
before the first one happens, not after.

**Privacy and partial visibility.** A bookshop is a benign example; the moment a role must *not* see
another role's facts, the "everyone holds the whole log" premise breaks, and filtered replication
becomes a security boundary rather than an efficiency one. `research-crux-integration.md` §9.2
already records that there is no authorship or authorization model anywhere in the specification.
This is the same gap arriving through a different door, and it will arrive.

---

## 11. What this exercise changed

Three things worth carrying back into the other documents:

1. **The unit of composition is the vertical slice — not the team, not the device, not the
   deployable** (§5). Teams and device roles are drivers, not units, and an architecture that bakes
   in a driver is wrong when the driver changes. Because a slice is a complete vertical coupled to
   its siblings only through the log, composition costs a `Cargo.toml` line, and most of what
   micro-frontends are reached for evaporates. This makes the slice contract in
   `research-crux-composition.md` §6 load-bearing rather than merely tidy, and it moves the shutdown
   test from being an acceptance criterion for separate Cores to being the acceptance criterion for
   slices.
2. **The slice manifest is the natural driver for filtered replication** (§5). SY-27 is currently
   open with no obvious consumer; this is one, and it suggests the answer is "filtered, driven by
   composition" rather than "whole-log."
3. **The convergence taxonomy is a modelling exercise, not a tax** (§7). Four of five real decisions
   in a real shop are safe offline, and the fifth maps onto a compensation the business already
   performs. That reframing is worth putting in front of anyone who hears "does not converge" and
   concludes the architecture is unsound.
