---
id: HS-S0021
uid: 52e6d8
type: story
slug: decision-model-composition
title: Consistency boundaries compose at compile time
parent: HS-P0011
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0020
blocks:
  - HS-S0025
  - HS-S0029
archetype: capability
slice: typed-vocabulary
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-16T20:02:43.349Z
links:
  pr: null
  commits:
    - 4dc6aeb
    - 3fde7d1
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# Consistency boundaries compose at compile time

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
