---
id: HS-S0056
uid: f48198
type: story
slug: wf-11-memory-ceiling-falsifier
title: WF-11's falsifier tested where the memory ceiling is real
parent: HS-P0013
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0054
blocks:
  - HS-S0058
archetype: capability
slice: evidence-and-verdicts
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-20T06:19:25.263Z
links:
  pr: null
  commits:
    - 14dbb4b
    - 4aa3820
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# WF-11's falsifier tested where the memory ceiling is real

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
