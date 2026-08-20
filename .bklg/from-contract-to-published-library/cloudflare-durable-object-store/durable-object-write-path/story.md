---
id: HS-S0050
uid: fabcd2
type: story
slug: durable-object-write-path
title: Append, head, contains_event_id and migrate against a real Durable Object
parent: HS-P0013
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0049
blocks:
  - HS-S0052
  - HS-S0053
archetype: capability
slice: real-worker-bindings
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-20T04:34:43.227Z
links:
  pr: null
  commits:
    - 3eb91cfcac8cfd85b4fc1e7b8e4d612d66866ff2
    - 2ea99fd10bc1af495c7e47b8bbd7116a04e1d28f
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# Append, head, contains_event_id and migrate against a real Durable Object

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
