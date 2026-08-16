---
id: HS-S0026
uid: 495a96
type: story
slug: projection-trait-and-runner
title: The application-facing Projection trait and its streaming runner
parent: HS-P0011
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0022
blocks:
  - HS-S0027
  - HS-S0028
  - HS-S0031
  - HS-S0032
archetype: capability
slice: projection-runner
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-16T19:59:10.752Z
links:
  pr: null
  commits:
    - 60b8072
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The application-facing Projection trait and its streaming runner

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
