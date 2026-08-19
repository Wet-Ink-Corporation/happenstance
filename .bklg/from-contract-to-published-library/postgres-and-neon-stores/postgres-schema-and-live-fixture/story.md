---
id: HS-S0061
uid: 3052fa
type: story
slug: postgres-schema-and-live-fixture
title: Migration 1, the live Postgres fixture, and the CI job that runs it
parent: HS-P0014
initiative: from-contract-to-published-library
project: postgres-and-neon-stores
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by: []
blocks:
  - HS-S0062
  - HS-S0071
archetype: foundation
slice: postgres-live-suite
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T05:10:09.790Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# Migration 1, the live Postgres fixture, and the CI job that runs it

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
