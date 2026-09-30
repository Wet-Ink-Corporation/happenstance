---
id: "kb-decision-wi-3c4f18"
title: "DDD aggregates"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Which prior mental model should boundaries-not-aggregates.md argue against — DDD aggregates, stream-per-entity, or none stated?\", facing the prior-model page is HS-I0007's open question; the landing page already contrasts with aggregates, we decided for DDD aggregates and neglected Stream-per-entity; None stated, on the premise that most readers come from DDD, accepting that if wrong: stream-per-entity readers find no bridge."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-09-30"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-3c4f18"
question: "Which prior mental model should boundaries-not-aggregates.md argue against — DDD aggregates, stream-per-entity, or none stated?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-30T02:54:01Z"
tree_hash: "196f0e0f336a42fd143c6a5c88bbbe30c0f559b4"
recommended: "A"
flip_condition: "evidence most arriving readers hold stream-per-entity"
---

# DDD aggregates

## Context and problem statement

Which prior mental model should boundaries-not-aggregates.md argue against — DDD aggregates, stream-per-entity, or none stated?

The prior-model page is HS-I0007's open question; the landing page already contrasts with aggregates.

Raised by an agent (marker) as a deferral and captured by Weigh-In as `wi-3c4f18`. Anchor: `runbook/phases/22-docs-site.md`.

## Decision drivers

- consistency with landing

## Considered options

### A · DDD aggregates (chosen)

Argue against the aggregate-plus-saga model, matching the landing page.

- Holds if most readers come from DDD.
- If wrong: stream-per-entity readers find no bridge.

### B · Stream-per-entity

Argue against one stream per entity, EventStoreDB-style.

- Holds if most readers come from EventStoreDB.
- If wrong: landing page and guide disagree.

### C · None stated

State that the teaching assumes no prior model.

- Holds if readers vary too widely.
- If wrong: BR-07's bridge stays missing.

## Evidence

- `site/templates/index.html`: Classical event sourcing asks you to choose the boundary when the schema is written: a Course aggregate or a Student aggregate.

## Decision outcome

Chosen option: **DDD aggregates**, the recommended option.

Decider's note: Argue against DDD aggregates, matching the landing page.

### Consequences

- Good, because it holds if most readers come from DDD.
- Bad, because if wrong: stream-per-entity readers find no bridge.

### Confirmation

Revisit when: evidence most arriving readers hold stream-per-entity

## Why this might be wrong

if the evaluator population is mostly EventStoreDB users

## Provenance

- Decided 2026-09-30T02:54:01Z by human:ryan (user-approved), via chat.
- Raised in session `f0834c2a-1fe5-44d1-9d05-77d50ee6b39b`.
- Verbatim: "kind: deferral"
