---
id: HS-S0035
uid: f39912
type: story
slug: adr-0022-append-condition-strategy
title: ADR-0022, written first and carrying a measured number
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0034
blocks:
  - HS-S0036
archetype: foundation
slice: bench-harness-and-adr
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-18T03:26:35.944Z
links:
  pr: null
  commits:
    - 791b929
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# ADR-0022, written first and carrying a measured number

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
