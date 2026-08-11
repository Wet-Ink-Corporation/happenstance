---
id: kb-decision-0008
title: One derivation scheme, both ports, and what a provided body owes
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0008
reversibility: low
phase: 1
supersedes: null
superseded_by: null
summary: >-
  ProjectionStore had copied EventStore's #[trait_variant::make] construction with no decision of
  its own, and PS-35 requires one document covering both ports. It gets one: a single derivation
  scheme serves both, and three rules bind any provided method on either. A provided body is
  hand-desugared to -> impl Future and never written as async fn. A body holding &self across an
  await takes where Self: Sync at the point of use and never in the attribute, because the
  attribute's whole bound list reaches read's stream and would reject any Cell-hiding adapter. And
  one body must compile against both the bare and the derived flavour, checked at the port. The
  ports diverge in the consequences: ProjectionStore's Batch GAT means a provided body cannot hold
  a batch across a suspension point under any remedy tried, and EventStore structurally cannot
  exhibit that. This decision also lifts ADR-0001's provisional marker, with full proof still owed
  by the phase-9 Durable Object adapter. Rejected: two hand-written traits per port, a different
  scheme per port, and Sync in the attribute. Four amendments are owed to the specification and
  none changes a normative MUST.
depends_on:
  - kb-decision-0001
related:
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - references/adr/0008-one-derivation-for-both-ports.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# One derivation scheme, both ports, and what a provided body owes

## Context

ADR-0001 chose `trait_variant` over two hand-written traits, but argued the choice only for
`EventStore`. `ProjectionStore` had carried the identical `#[trait_variant::make(SendProjectionStore:
Send)]` construction since it was written, decided by nobody, and `SPECIFICATION.md`'s PS-35
requires one document to cover both ports because an application holding a `!Send` projection
store beside a `Send` event store is the ordinary case, not the exotic one. A second fact had also
gone unstated: `trait_variant` clones a provided method's body into the derived variant, so one
body must type-check under both flavours' bounds at once. ADR-0001's own evidence base was a
`cargo check` with zero `!Send` implementers anywhere in the tree — a compile of a trait, not of
an implementation.

## Decision

One scheme, both ports, derived — `ProjectionStore` is now covered by the same reasoning as
`EventStore` rather than by inheritance from a file it was copied out of. Three rules bind any
provided method: it is hand-desugared to `-> impl Future`, never written `async fn`, because the
cloned body loses its asyncness under that spelling and fails at the port; a body holding `&self`
across an `await` takes `where Self: Sync` at the point of use and never in the attribute, because
the attribute's bound list also reaches `read`'s stream and would reject any adapter whose stream
hides a `Cell`; and one body must compile against both flavours, checked at the port, so a body
that compiles under `EventStore` and not `SendEventStore` is a compile error discovered by whoever
adds the first defaulted method rather than by whoever designed the port.

The two ports diverge in what the scheme costs. `ProjectionStore`'s `Batch<'a>` is a GAT, and a
provided body that holds a batch across a suspension point does not compile under any remedy
tried — no bound spelling escapes it, and `EventStore` structurally cannot exhibit the failure
because it has no equivalent GAT. This is the finding a single-port ADR could not have surfaced.

This decision also lifts ADR-0001's `provisional` marker: its lift condition, a `!Send` reference
store passing the conformance suite, is now met in-tree by a `Rc<RefCell<…>>` store implementing
`EventStore` directly. Full proof remains the phase-9 Cloudflare Durable Object adapter — what is
proved here is that the design admits a `!Send` implementer, not that a real platform SDK fits.

## Consequences and alternatives rejected

Good: one definition per port, both targets, and `SendProjectionStore` is covered by an ADR for
the first time. Bad: provided methods are second-class on the bare flavour — any method a `!Send`
adapter might plausibly call should be required, or take its arguments as parameters rather than
close over `&self`. Bad and load-bearing for phase 6: a `ProjectionStore` provided method may
never hold its batch across a suspension point, and phase 6 must decide whether the GAT survives
knowing that. `Error`'s bound stays deliberately undecided here, left to the decision this one
extends.

Rejected: two hand-written traits per port, which loses the blanket impl that makes binding the
weaker flavour work at every call site; a different scheme per port, which would let
`ProjectionStore`'s GAT carry `Batch: Send` on one flavour only but costs a second ADR governing
one seam; and `Sync` in the attribute rather than at the point of use, which reaches `read`'s
stream and rejects any adapter hiding a `Cell`. Four amendments are owed to the specification's
citations and rule adequacy — none changes a normative MUST.
