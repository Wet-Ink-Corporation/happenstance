---
id: kb-decision-0103
title: The ProjectionStore freeze accepts the Async flavour axis with no !Send adapter at its far end
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0103
reversibility: high
phase: 13
supersedes: null
superseded_by: null
summary: >-
  Accepted on 2026-11-08 for #125, on the owner's decision of that day. The frozen ProjectionStore
  port sits on CF-25's Async flavour axis, and nothing stands at its far end: no !Send projection
  adapter exists or is planned before 1.0. The risk is accepted by name, axis and port, as CF-25's
  second route allows. The Send flavour, SendProjectionStore, is derived mechanically by
  trait_variant from the bare trait, as EventStore's is. A !Send projection adapter, such as a
  Durable Object ProjectionStore, running the projection suite retires the acceptance.
depends_on:
  - kb-decision-0001
  - kb-decision-0008
  - kb-decision-0063
related:
  - kb-decision-0013
source_paths:
  - spec/SPECIFICATION.md
  - crates/happenstance-core/src/projection.rs
  - xtask/src/spec_trace/portfolio.rs
last_reviewed: 2026-11-08
---

# The ProjectionStore freeze accepts the Async flavour axis with no !Send adapter at its far end

**Accepted on 2026-11-08 for #125**, on the owner's decision of that day in the project thread.
No long-form record exists; this atom is the record.

## Context

CF-25 forbids a port carrying `[FROZEN]` clauses while an axis it sits on has no passing
implementation at its far end, unless the freeze names the axis and the record accepting the risk.
#125 built the check, `cargo xtask spec-trace`, against §6.5's portfolio and its freeze ledger. The
check found the `ProjectionStore` row for **Async flavour** undischarged:

- ADR-0063 froze the port on the **Batch shape** axis and names no other.
- Every projection adapter that has run the projection suite is `Send`: memory, SQLite, both
  Postgres stores and Neon. `happenstance-cloudflare`, the workspace's `!Send` adapter, has no
  projection store.
- No open `PS` clause carries the exposure.

## Decision

The `ProjectionStore` freeze accepts the **Async flavour** axis with nothing at its far end. §6.5's
freeze ledger discharges the row `Async flavour | ProjectionStore` by this record.

The acceptance rests on three facts:

1. **No `!Send` projection adapter exists or is planned before 1.0.** No deployment in the
   workspace's scenarios needs one before then.
2. **The flavour split is mechanical.** `ProjectionStore` is declared once without a `Send` bound,
   and `trait_variant` derives `SendProjectionStore` from it (ADR-0001, ADR-0008). That is the same
   derivation `EventStore` uses, and `EventStore`'s bare flavour has run its suite under `workerd`
   since phase 9. So the port's shape does not depend on `Send`. What is unproven is that a real
   `!Send` storage engine can implement it.
3. **The remedy is additive.** A `!Send` adapter that cannot implement the port as frozen would
   show up as a refusal from the trait's existing bare flavour. Nothing about the `Send` flavour
   would need to move for a `!Send` adapter to be built.

## What retires it

A `!Send` projection adapter running the projection suite and passing it. A Durable Object
`ProjectionStore` in `happenstance-cloudflare` is the obvious candidate. Then the ledger row moves
to `far end`, and the portfolio's verdict for the axis names that adapter.

## What reopens it

A `!Send` projection adapter that cannot pass the projection suite through the bare
`ProjectionStore` flavour. That would mean the port froze against `Send` storage alone. It would
need a new record, and probably a change to the port.

## Alternatives

- **Build the far end now.** A Durable Object projection store is real work in a crate whose
  roadmap does not call for one before 1.0. It is the route that retires this record, not a
  precondition for the freeze.
- **Carry it by an open `PS` clause.** No `PS` clause is about the flavour split. Minting one only
  to carry the row would disclose the exposure in the wrong place.
- **Record the port as not sitting on the axis.** False: the port has both flavours, so it sits on
  the axis.

## Consequences

- The ledger row `Async flavour | ProjectionStore` reads `ADR-0103`, and `spec-trace` holds it to
  this record's `status: accepted` and to the record naming both the axis and the port.
- Superseding this record without retiring the risk turns the row red again.
