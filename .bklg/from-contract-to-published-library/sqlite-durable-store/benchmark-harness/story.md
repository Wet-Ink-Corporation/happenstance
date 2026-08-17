---
id: HS-S0034
uid: e3a982
type: story
slug: benchmark-harness
title: event_store_benchmarks! in the testkit, provably not conformance
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by: []
blocks:
  - HS-S0035
archetype: foundation
slice: bench-harness-and-adr
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-17T12:59:40.973Z
links:
  pr: null
  commits:
    - "2665883"
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# event_store_benchmarks! in the testkit, provably not conformance

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
