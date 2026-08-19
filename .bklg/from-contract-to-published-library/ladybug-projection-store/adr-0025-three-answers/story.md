---
id: HS-S0075
uid: 98ffb8
type: story
slug: adr-0025-three-answers
title: "ADR-0025: checkpoint placement, mutation vocabulary, blocking bridge"
parent: HS-P0015
initiative: from-contract-to-published-library
project: ladybug-projection-store
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0074
blocks:
  - HS-S0076
archetype: foundation
slice: preflight-and-decisions
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T05:10:27.848Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# ADR-0025: checkpoint placement, mutation vocabulary, blocking bridge

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
