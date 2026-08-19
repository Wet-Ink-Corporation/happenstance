---
id: HS-S0006
uid: 57b32c
type: story
slug: memory-projection-store
title: MemoryProjectionStore as oracle, doctest target and cold-start fix
parent: HS-P0010
initiative: from-contract-to-published-library
project: projection-store-freeze
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0004
  - HS-S0005
blocks:
  - HS-S0007
archetype: foundation
slice: projection-port-and-probe
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-14T17:01:00.363Z
links:
  pr: null
  commits:
    - 5fd62c6
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# MemoryProjectionStore as oracle, doctest target and cold-start fix

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
