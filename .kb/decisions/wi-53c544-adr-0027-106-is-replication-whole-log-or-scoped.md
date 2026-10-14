---
id: "kb-decision-wi-53c544"
title: "ADR-0027 (#106): is replication whole-log or scoped at 1.0 (SY-27/28/29): A"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"ADR-0027 (#106): is replication whole-log or scoped at 1.0 (SY-27/28/29)?\", facing the question an agent raised, we decided for A."
depends_on: []
related:
  - kb-decision-0027
source_paths: []
last_reviewed: "2026-10-14"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-53c544"
question: "ADR-0027 (#106): is replication whole-log or scoped at 1.0 (SY-27/28/29)?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-14T01:09:50Z"
tree_hash: "2f25eccf8841e601a8ba820350af089070f5f579"
---

# ADR-0027 (#106): is replication whole-log or scoped at 1.0 (SY-27/28/29): A

## Context and problem statement

ADR-0027 (#106): is replication whole-log or scoped at 1.0 (SY-27/28/29)?

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-53c544`. Anchor: `spec/SPECIFICATION.md:7768`.

## Decision outcome

Chosen option: **A**.

Decider's note: Ryan 2026-10-14: 'Whole-log'. Whole-log at 1.0; a confidentiality slice is its own store; scoped replication may follow post-1.0 as an additive opt-in with its own ADR. SY-29 Query and #122 leave the 1.0 path.

## Provenance

- Decided 2026-10-14T01:09:50Z by human:ryan (user-directed), via chat.
- Raised in session `2fe67e8e-8afc-5d1f-abc9-f48820905de5`.
- Verbatim: "ADR-0027 (#106): is replication whole-log or scoped at 1.0 (SY-27/28/29)?"
