---
id: HS-S0041
uid: 93b060
type: story
slug: concurrency-family-and-contender-count
title: The concurrency family green, and the 8-versus-64 discrepancy closed
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0040
blocks:
  - HS-S0042
archetype: capability
slice: race-model-and-durability
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-18T13:28:18.115Z
links:
  pr: null
  commits:
    - 995b987
    - 071bc2f
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# The concurrency family green, and the 8-versus-64 discrepancy closed

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
