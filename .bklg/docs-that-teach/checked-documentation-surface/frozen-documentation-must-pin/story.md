---
id: HS-S0143
uid: ece2d2
type: story
slug: frozen-documentation-must-pin
title: The frozen documentation MUSTs are enumerated by clause id in one place
parent: HS-P0020
initiative: docs-that-teach
project: checked-documentation-surface
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0138
  - HS-S0141
blocks:
  - HS-S0145
archetype: capability
slice: specification-pin
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-17T22:02:17.626Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The frozen documentation MUSTs are enumerated by clause id in one place

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
