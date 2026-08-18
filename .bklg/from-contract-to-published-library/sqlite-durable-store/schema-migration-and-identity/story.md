---
id: HS-S0036
uid: d93f1a
type: story
slug: schema-migration-and-identity
title: "Migration 1: the amended schema, persisted StoreId, and idempotent concurrent open"
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: in-progress
process: story
stage: implement
automation: HITL
severity: null
blocked_by:
  - HS-S0035
blocks:
  - HS-S0037
  - HS-S0038
  - HS-S0044
archetype: foundation
slice: durable-event-store
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-18T03:27:23.902Z
links:
  pr: null
  commits:
    - 8381c89
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# Migration 1: the amended schema, persisted StoreId, and idempotent concurrent open

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
