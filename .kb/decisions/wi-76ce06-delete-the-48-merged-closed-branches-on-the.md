---
id: "kb-decision-wi-76ce06"
title: "Allow the agent"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Delete the 48 merged/closed branches on the remote (#74)?\", facing 48 merged or closed branches remain on the remote; the agent cannot delete them, we decided for Allow the agent and neglected You delete them; Keep them, on the premise that you want agents to prune branches routinely, accepting that if wrong: agents gain a destructive git permission beyond this task."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-76ce06-delete-the-48-merged-closed-branches-on-the.md
last_reviewed: "2026-10-10"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-76ce06"
question: "Delete the 48 merged/closed branches on the remote (#74)?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-10T18:42:53Z"
tree_hash: "369163dec409bfc6fb5ef6afa83333663184bce7"
recommended: "A"
flip_condition: "wanting agents to prune branches unattended every session"
---

# Allow the agent

## Context and problem statement

Delete the 48 merged/closed branches on the remote (#74)?

48 merged or closed branches remain on the remote; the agent cannot delete them.

Raised by an agent (marker) as a deferral and captured by Weigh-In as `wi-76ce06`. Anchor: `runbook/handover.md:78`.

## Decision drivers

- effort for you
- standing risk

## Considered options

### A · You delete them

Run the one-liner posted on #74, and turn on auto-delete of head branches

- Holds if you have a minute with your own git login.
- If wrong: nothing lost: every tip stays as refs/pull/N/head.

### B · Allow the agent (chosen)

Add a permission rule for git push --delete, then I delete them

- Holds if you want agents to prune branches routinely.
- If wrong: agents gain a destructive git permission beyond this task.

### C · Keep them

Close #74 as not planned

- Holds if the branch names are useful history.
- If wrong: the list grows by one per PR.

## Evidence

- `runbook/handover.md:76`: every one of them points at exactly the head of a merged PR, or of spike #40 or #50
- `runbook/handover.md:78`: Agent sessions are refused the delete. Use your own login

## Decision outcome

Chosen option: **Allow the agent**, overriding the recommendation (A).

Decider's note: Allow the agent to delete the merged branches.

### Consequences

- Good, because it holds if you want agents to prune branches routinely.
- Bad, because if wrong: agents gain a destructive git permission beyond this task.

### Confirmation

Revisit when: wanting agents to prune branches unattended every session

## Why this might be wrong

if pruning recurs often, B saves repeated manual work

## Provenance

- Decided 2026-10-10T18:42:53Z by human:ryan (user-directed), via chat.
- Raised in session `6c356295-9b73-5e6b-9e5a-c2ca6d15d8f1`.
- Verbatim: "kind: deferral"
