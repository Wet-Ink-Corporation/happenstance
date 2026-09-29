---
id: "kb-decision-wi-d61f21"
title: "workerd sibling job before 1.0"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Does a workerd-class conformance run land before 1.0, so Cloudflare's 1.0 claim is conformance on the real runtime?\", facing it is part of what 1.0 promises, and ADR-0066 records it, we decided for workerd sibling job before 1.0 and neglected Scope the claim to the SqlStorage double, on the premise that the job can be kept green at acceptable cost, accepting that if wrong: about 1-2 days plus ongoing upkeep."
depends_on: []
related:
  - kb-decision-0066
source_paths:
  - .kb/_intake/decisions/wi-d61f21-does-a-workerd-class-conformance-run-land.md
last_reviewed: "2026-09-29"
reversibility: low
phase: 16
supersedes: null
superseded_by: null
weighin_item: "wi-d61f21"
question: "Does a workerd-class conformance run land before 1.0, so Cloudflare's 1.0 claim is conformance on the real runtime?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-09-29T15:31:09Z"
tree_hash: "47e887d507d1f5b2fa472f496b10c03e8c87fd3c"
recommended: "A"
---

# workerd sibling job before 1.0

## Context and problem statement

Does a workerd-class conformance run land before 1.0, so Cloudflare's 1.0 claim is conformance on the real runtime?

It is part of what 1.0 promises, and ADR-0066 records it.

Raised by an agent (manual) as a question and captured by Weigh-In as `wi-d61f21`. Anchor: `.kb/open-questions/no-workerd-class-runner-in-the-gate.md:97`.

## Considered options

### A · Scope the claim to the SqlStorage double

1.0 promises conformance against the node:sqlite shim; workerd after 1.0

- Holds if no one needs platform guarantees at 1.0.
- If wrong: ADR-0052 widths freeze unmeasured.

### B · workerd sibling job before 1.0 (chosen)

A CI job shaped like live-postgres runs every rule under workerd; the SQL-text wall and partition widths are measured before they are promised

- Holds if the job can be kept green at acceptable cost.
- If wrong: about 1-2 days plus ongoing upkeep.

## Evidence

- `.kb/open-questions/no-workerd-class-runner-in-the-gate.md:97`: the item this decision settles

## Decision outcome

Chosen option: **workerd sibling job before 1.0**, overriding the recommendation (A).

Decider's note: the owner chose B after the scope of the job was explained (phase-16 planning session, 2026-09-29)

### Consequences

- Good, because it holds if the job can be kept green at acceptable cost.
- Bad, because if wrong: about 1-2 days plus ongoing upkeep.

### Confirmation

Revisit if the premises above stop holding.

## Why this might be wrong

see ADR-0066

## Provenance

- Decided 2026-09-29T15:31:09Z by human:ryan (user-directed), via chat.
- Raised in session `1a19c172-8a8c-448d-ad9a-ce7ebf44140f`.
- Verbatim: "Does a workerd-class conformance run land before 1.0, so Cloudflare's 1.0 claim is conformance on the real runtime?"
