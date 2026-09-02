---
id: HS-S0071
uid: 59bb33
type: story
slug: postgres-projection-store
title: PostgresProjectionStore against the frozen Batch
parent: HS-P0014
initiative: from-contract-to-published-library
project: postgres-and-neon-stores
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0061
blocks:
  - HS-S0072
archetype: capability
slice: postgres-projection-store
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T05:10:24.669Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# PostgresProjectionStore against the frozen Batch

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
