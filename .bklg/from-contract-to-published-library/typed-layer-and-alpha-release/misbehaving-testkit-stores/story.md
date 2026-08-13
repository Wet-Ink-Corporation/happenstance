---
id: HS-S0024
uid: 9f617e
type: story
slug: misbehaving-testkit-stores
title: FaultyStore<S> and GappyMemoryStore in the testkit
parent: HS-P0011
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by: []
blocks:
  - HS-S0025
archetype: capability
slice: testing-surface
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T05:09:20.731Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# FaultyStore<S> and GappyMemoryStore in the testkit

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
