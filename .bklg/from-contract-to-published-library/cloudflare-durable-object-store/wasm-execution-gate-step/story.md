---
id: HS-S0048
uid: 67dadc
type: story
slug: wasm-execution-gate-step
title: A wasm32 conformance run inside cargo xtask ci, not beside it
parent: HS-P0013
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by: []
blocks:
  - HS-S0054
archetype: foundation
slice: wasm-execution-seam
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-19T17:24:37.626Z
links:
  pr: null
  commits:
    - 8ea7bb7c8b8d282af73cb08f860d948367a85a8e
    - d1bae6d0ab21f900f6d17bea2a9e58a75743afc4
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# A wasm32 conformance run inside cargo xtask ci, not beside it

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
