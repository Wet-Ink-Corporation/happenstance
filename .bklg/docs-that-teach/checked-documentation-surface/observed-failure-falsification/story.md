---
id: HS-S0144
uid: 067ef4
type: story
slug: observed-failure-falsification
title: The gate is watched failing on a page broken on purpose, then recovering
parent: HS-P0020
initiative: docs-that-teach
project: checked-documentation-surface
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0136
  - HS-S0138
blocks:
  - HS-S0145
archetype: capability
slice: falsification-and-limits
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-18T15:18:29.267Z
links:
  pr: null
  commits:
    - b0bb9bd
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The gate is watched failing on a page broken on purpose, then recovering

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
