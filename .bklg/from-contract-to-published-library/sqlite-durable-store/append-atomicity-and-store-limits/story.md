---
id: HS-S0037
uid: 0fccd1
type: story
slug: append-atomicity-and-store-limits
title: "append: preconditions, then one BEGIN IMMEDIATE transaction"
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0036
blocks:
  - HS-S0040
archetype: capability
slice: durable-event-store
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T05:09:39.209Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# append: preconditions, then one BEGIN IMMEDIATE transaction

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
