---
id: "kb-decision-wi-8cdf0a"
title: "Keep as draft"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Should the four reconciled personas be promoted into .kb/product/ now, as the approved plan said, or stay as a draft in runbook/phases/22-docs-site.md until phase 20's friction log observes one?\", facing the approved plan said to promote personas to .kb/product/; the layer's README forbids unevidenced ones, we decided for Keep as draft and neglected Promote now, on the premise that the layer's rule stands, accepting that if wrong: the next initiative re-derives them."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-8cdf0a-should-the-four-reconciled-personas-be-promoted.md
last_reviewed: "2026-09-30"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-8cdf0a"
question: "Should the four reconciled personas be promoted into .kb/product/ now, as the approved plan said, or stay as a draft in runbook/phases/22-docs-site.md until phase 20's friction log observes one?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-30T02:54:01Z"
tree_hash: "196f0e0f336a42fd143c6a5c88bbbe30c0f559b4"
recommended: "A"
flip_condition: "a friction-log observation"
---

# Keep as draft

## Context and problem statement

Should the four reconciled personas be promoted into .kb/product/ now, as the approved plan said, or stay as a draft in runbook/phases/22-docs-site.md until phase 20's friction log observes one?

The approved plan said to promote personas to .kb/product/; the layer's README forbids unevidenced ones.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-8cdf0a`. Anchor: `runbook/phases/22-docs-site.md`.

## Decision drivers

- rule

## Considered options

### A · Keep as draft (chosen)

Stay in phase 22 until phase 20's friction log observes one.

- Holds if the layer's rule stands.
- If wrong: the next initiative re-derives them.

### B · Promote now

Write concept + playbook atoms with the unobserved qualification.

- Holds if you accept inferred personas.
- If wrong: a guess gains a finding's standing.

## Evidence

- `.kb/product/README.md`: Promoting it early gives a guess the standing of a finding

## Decision outcome

Chosen option: **Keep as draft**, the recommended option.

Decider's note: Keep as draft in phase 22 until the friction log observes one.

### Consequences

- Good, because it holds if the layer's rule stands.
- Bad, because if wrong: the next initiative re-derives them.

### Confirmation

Revisit when: a friction-log observation

## Why this might be wrong

if the rule was meant for redkiln-era process only

## Provenance

- Decided 2026-09-30T02:54:01Z by human:ryan (user-approved), via chat.
- Raised in session `f0834c2a-1fe5-44d1-9d05-77d50ee6b39b`.
- Verbatim: "Should the four reconciled personas be promoted into .kb/product/ now, as the approved plan said, or stay as a draft in runbook/phases/22-docs-site.md until phase 20's friction log observes one?"
