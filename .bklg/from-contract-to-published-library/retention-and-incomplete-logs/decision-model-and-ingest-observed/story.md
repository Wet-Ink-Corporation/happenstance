---
id: HS-S0118
uid: b65398
type: story
slug: decision-model-and-ingest-observed
title: "Two readers meet the hole: read_decision_model and IngestStore::holds"
parent: HS-P0018
initiative: from-contract-to-published-library
project: retention-and-incomplete-logs
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0114
blocks:
  - HS-S0120
  - HS-S0121
archetype: capability
slice: readers-against-the-hole
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T14:12:18.271Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# Two readers meet the hole: read_decision_model and IngestStore::holds

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
