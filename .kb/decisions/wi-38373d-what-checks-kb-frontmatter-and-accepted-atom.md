---
id: "kb-decision-wi-38373d"
title: "What checks .kb frontmatter and accepted-atom immutability until redkiln-rs: xtask lint"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"What checks .kb frontmatter and accepted-atom immutability until redkiln-rs?\", facing the question an agent raised, we decided for xtask lint."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-38373d-what-checks-kb-frontmatter-and-accepted-atom.md
last_reviewed: "2026-09-29"
reversibility: high
phase: 15
supersedes: null
superseded_by: null
weighin_item: "wi-38373d"
question: "What checks .kb frontmatter and accepted-atom immutability until redkiln-rs?"
door: two-way
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-09-29T03:09:40Z"
tree_hash: "0b4e8c0bae04680be8e1de29015fc8bb175f0d64"
---

# What checks .kb frontmatter and accepted-atom immutability until redkiln-rs: xtask lint

## Context and problem statement

What checks .kb frontmatter and accepted-atom immutability until redkiln-rs?

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-38373d`. Anchor: `runbook/phases/15-reconcile.md:51`.

## Decision outcome

Chosen option: **xtask lint**.

Decider's note: Owner's answer, 2026-09-28: an xtask lint diffing every status: accepted atom body against the merge base, run in the gate.

## Provenance

- Decided 2026-09-29T03:09:40Z by human:ryan (user-directed), via chat.
- Raised in session `03f75530-aeee-435f-b13a-f68fbde2524c`.
- Verbatim: "What checks .kb frontmatter and accepted-atom immutability until redkiln-rs?"
