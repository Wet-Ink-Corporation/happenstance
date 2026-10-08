---
id: "kb-decision-wi-6ae424"
title: "Re-add now"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Once the fence merges, will you re-add `conformance against a live Neon endpoint` to the `Protect main` ruleset (id 22926481) as a required check (ADR-0087 D8, wi-0f1291)?\", facing the live Neon job was dropped from required checks while ES-11 failed on it; the fence that fixes it is on main, we decided for Re-add now and neglected Keep non-required, on the premise that the job is green on main, accepting that if wrong: a Neon outage blocks merges; drop it again."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-08"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-6ae424"
question: "Once the fence merges, will you re-add `conformance against a live Neon endpoint` to the `Protect main` ruleset (id 22926481) as a required check (ADR-0087 D8, wi-0f1291)?"
door: two-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-08T12:57:33Z"
tree_hash: "62904db838806d6ee58397ad1f39d294acf908da"
recommended: "A"
flip_condition: "the job going red on main after the fence"
---

# Re-add now

## Context and problem statement

Once the fence merges, will you re-add `conformance against a live Neon endpoint` to the `Protect main` ruleset (id 22926481) as a required check (ADR-0087 D8, wi-0f1291)?

The live Neon job was dropped from required checks while ES-11 failed on it; the fence that fixes it is on main.

Raised by an agent (marker) as a deferral and captured by Weigh-In as `wi-6ae424`. Anchor: `runbook/handover.md:62`.

## Decision drivers

- guard

## Considered options

### A · Re-add now (chosen)

Make the Neon job required in ruleset 22926481

- Holds if the job is green on main.
- If wrong: a Neon outage blocks merges; drop it again.

### B · Keep non-required

Leave it advisory

- Holds if Neon's endpoint is too flaky to gate on.
- If wrong: an ES-11 regression merges silently.

## Evidence

- `runbook/handover.md:62`: When L8 lands, the owner re-adds `conformance against a live Neon endpoint`

## Decision outcome

Chosen option: **Re-add now**, the recommended option.

Decider's note: re-add the live Neon check to ruleset 22926481 now

### Consequences

- Good, because it holds if the job is green on main.
- Bad, because if wrong: a Neon outage blocks merges; drop it again.

### Confirmation

Revisit when: the job going red on main after the fence

## Why this might be wrong

one green run on main is a thin sample

## Provenance

- Decided 2026-10-08T12:57:33Z by human:ryan (user-approved), via chat.
- Raised in session `6ec241b3-3771-51a8-b5a3-78697abb6042`.
- Verbatim: "kind: deferral"
