---
id: "kb-decision-wi-ab0a5a"
title: "How much merge authority does the phase 15 AFK session have over its own PRs: Self-merge on green"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"How much merge authority does the phase 15 AFK session have over its own PRs?\", facing the question an agent raised, we decided for Self-merge on green."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-ab0a5a-how-much-merge-authority-does-the-phase-15-afk.md
last_reviewed: "2026-09-29"
reversibility: high
phase: 15
supersedes: null
superseded_by: null
weighin_item: "wi-ab0a5a"
question: "How much merge authority does the phase 15 AFK session have over its own PRs?"
door: two-way
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-09-29T03:09:40Z"
tree_hash: "0b4e8c0bae04680be8e1de29015fc8bb175f0d64"
---

# How much merge authority does the phase 15 AFK session have over its own PRs: Self-merge on green

## Context and problem statement

How much merge authority does the phase 15 AFK session have over its own PRs?

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-ab0a5a`. Anchor: `runbook/handover.md:1`.

## Decision outcome

Chosen option: **Self-merge on green**.

Decider's note: One PR per item, squash-merged when green. One re-run of the failed jobs for a failure in code the PR did not touch; a second failure stops and is reported.

## Provenance

- Decided 2026-09-29T03:09:40Z by human:ryan (user-directed), via chat.
- Raised in session `03f75530-aeee-435f-b13a-f68fbde2524c`.
- Verbatim: "How much merge authority does the phase 15 AFK session have over its own PRs?"
