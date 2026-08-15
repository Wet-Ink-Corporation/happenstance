---
id: HS-S0011
uid: 5261f4
type: story
slug: reset-rules
title: reset is one unit of work, scoped, refusable, and not commit-at-FIRST
parent: HS-P0010
initiative: from-contract-to-published-library
project: projection-store-freeze
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0008
  - HS-S0009
blocks:
  - HS-S0013
archetype: capability
slice: reset-and-rebuild-rules
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-15T19:05:19.371Z
links:
  pr: null
  commits:
    - 10ace94
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# reset is one unit of work, scoped, refusable, and not commit-at-FIRST

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
