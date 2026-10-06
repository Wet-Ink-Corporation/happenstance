---
id: "kb-decision-wi-557b41"
title: "Fix the token, re-run, then merge"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should the deployed leg wait for the Cloudflare token's IP filter to be lifted, or should\", facing the deployed leg is the only measurement of the platform row wall, and CI's token is refused by an IP filter, we decided for Fix the token, re-run, then merge and neglected Merge #34 red now; deployed leg follows, on the premise that the token fix takes minutes, accepting that if wrong: L6b waits on a credential."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-02"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-557b41"
question: "Should the deployed leg wait for the Cloudflare token's IP filter to be lifted, or should"
door: two-way
blast_radius: system
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-02T03:03:56Z"
tree_hash: "67759e9eb6f9286f57468ac553b7fc5c09cf4abc"
recommended: "A"
flip_condition: "the token cannot be changed this week"
---

# Fix the token, re-run, then merge

## Context and problem statement

Should the deployed leg wait for the Cloudflare token's IP filter to be lifted, or should

The deployed leg is the only measurement of the platform row wall, and CI's token is refused by an IP filter.

Raised by an agent (marker) as a deferral and captured by Weigh-In as `wi-557b41`. Anchor: `runbook/handover.md:46`.

## Decision drivers

- evidence before merge

## Considered options

### A · Fix the token, re-run, then merge (chosen)

lift the IP filter, confirm Workers Scripts: Edit, re-run #34

- Holds if the token fix takes minutes.
- If wrong: L6b waits on a credential.

### B · Merge #34 red now; deployed leg follows

merge with only the local leg observed; record the deployed run later

- Holds if the deployed numbers only matter for 17b's metadata ceiling.
- If wrong: main carries a job red for two reasons.

## Evidence

- `CI job 110683349880`: Cannot use the access token from location: 172.212.163.227 [code: 9109]
- `runbook/handover.md:46`: The local leg is red in CI as intended

## Decision outcome

Chosen option: **Fix the token, re-run, then merge**, the recommended option.

Decider's note: Fix token first (lift the IP filter, confirm Workers Scripts: Edit), re-run #34, then merge.

### Consequences

- Good, because it holds if the token fix takes minutes.
- Bad, because if wrong: L6b waits on a credential.

### Confirmation

Revisit when: the token cannot be changed this week

## Why this might be wrong

the token permissions may need more than the IP filter

## Provenance

- Decided 2026-10-02T03:03:56Z by human:ryan (user-approved), via chat.
- Raised in session `a2f7a327-66cd-4d89-9cfc-eed8abed1ccf`.
- Verbatim: "kind: deferral"
