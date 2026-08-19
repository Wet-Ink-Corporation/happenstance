---
id: HS-S0049
uid: 8d04b0
type: story
slug: worker-binding-layer
title: The real Durable Object SqlStorage bindings replace the stand-in
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
  - HS-S0050
  - HS-S0051
  - HS-S0059
archetype: foundation
slice: real-worker-bindings
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-19T21:58:17.391Z
links:
  pr: null
  commits:
    - 310a4c8bbaa10032677f8137f701a0a45e14e0a0
    - 2ea99fd10bc1af495c7e47b8bbd7116a04e1d28f
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The real Durable Object SqlStorage bindings replace the stand-in

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
