---
id: "kb-decision-d0dce7c81f"
title: "REMOVAL + remove_outside_port"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"What name and shape should #147's Fixture removal capability take, and does Cloudflare grant it?\", facing a new item on the published Fixture trait is named once, we decided for REMOVAL + remove_outside_port and neglected PRUNE + prune; DELETION + delete_events, on the premise that CF host fixture can run raw SQL, accepting that if wrong: CF declines instead, with the VT-6 answer in its reason."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-4a43c4a6b6"
question: "What name and shape should #147's Fixture removal capability take, and does Cloudflare grant it?"
door: one-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T05:34:44Z"
tree_hash: "e3e026bfb7960b8cb224028372d3bb417c203567"
recommended: "A"
flip_condition: "CF fixture lacks raw SQL"
---

# REMOVAL + remove_outside_port

## Context and problem statement

What name and shape should #147's Fixture removal capability take, and does Cloudflare grant it?

A new item on the published Fixture trait is named once.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-4a43c4a6b6`. Anchor: `crates/happenstance-testkit/src/contract.rs:342`.

## Considered options

### A · REMOVAL + remove_outside_port (chosen)

Declined by default; CF grants via raw DELETE, delete_all() documented as a different store (VT-6)

- Holds if CF host fixture can run raw SQL.
- If wrong: CF declines instead, with the VT-6 answer in its reason.

### B · PRUNE + prune

Same shape, prune-flavoured name

- Holds if the main use is device pruning.
- If wrong: misnames scattered purges.

### C · DELETION + delete_events

Same shape, deletion name

- Holds if plain naming wins.
- If wrong: reads as port deletion ADR-0028 refuses.

## Evidence

- `crates/happenstance-testkit/src/contract.rs:342`: const MID_BATCH_FAULT: Capability = Capability::declined(...)

## Decision outcome

Chosen option: **REMOVAL + remove_outside_port**, the recommended option.

### Consequences

- Good, because it holds if CF host fixture can run raw SQL.
- Bad, because if wrong: CF declines instead, with the VT-6 answer in its reason.

### Confirmation

Revisit when: CF fixture lacks raw SQL

## Why this might be wrong

long name

## Provenance

- Decided 2026-11-15T05:34:44Z by human:claude (user-approved), via cli.
- Raised in session `456d08b0-9a08-5fef-92d6-6305f2ca971f`.
- Verbatim: "What name and shape should #147's Fixture removal capability take, and does Cloudflare grant it?"
