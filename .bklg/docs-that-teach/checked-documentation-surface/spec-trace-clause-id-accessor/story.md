---
id: HS-S0141
uid: 0d3b02
type: story
slug: spec-trace-clause-id-accessor
title: One resolver for specification clause ids, next to the parser that owns them
parent: HS-P0020
initiative: docs-that-teach
project: checked-documentation-surface
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by: []
blocks:
  - HS-S0142
  - HS-S0143
archetype: foundation
slice: specification-pin
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-17T22:02:16.486Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# One resolver for specification clause ids, next to the parser that owns them

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
