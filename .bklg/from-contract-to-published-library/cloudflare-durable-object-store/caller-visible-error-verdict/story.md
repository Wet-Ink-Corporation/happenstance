---
id: HS-S0052
uid: c1b6b5
type: story
slug: caller-visible-error-verdict
title: "The ES-6 artefact: constraint violation versus transport fault, recovered by a caller"
parent: HS-P0013
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0050
blocks:
  - HS-S0058
archetype: capability
slice: real-worker-bindings
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-19T21:58:33.247Z
links:
  pr: null
  commits:
    - 5955cb3fb5c8687447a7beda3aa8890d842e8a97
    - 2ea99fd10bc1af495c7e47b8bbd7116a04e1d28f
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The ES-6 artefact: constraint violation versus transport fault, recovered by a caller

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
