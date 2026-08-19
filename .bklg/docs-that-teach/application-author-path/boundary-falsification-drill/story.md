---
id: HS-S0186
uid: a14082
type: story
slug: boundary-falsification-drill
title: Removing the boundary makes the repository fail
parent: HS-P0022
initiative: docs-that-teach
project: application-author-path
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0185
blocks:
  - HS-S0189
  - HS-S0190
archetype: capability
slice: opening-encounter
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-19T20:01:30.590Z
links:
  pr: null
  commits:
    - cc9c4a4
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# Removing the boundary makes the repository fail

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
