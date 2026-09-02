---
id: HS-S0051
uid: d1a719
type: story
slug: durable-object-read-path
title: "A lazy read that is still one sample: ADR-0011's ceiling-and-page"
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
  - HS-S0053
  - HS-S0058
archetype: capability
slice: real-worker-bindings
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-20T04:36:10.505Z
links:
  pr: null
  commits:
    - 98913200d797f7cd52a1ae17764cca6bbb8f086d
    - 2ea99fd10bc1af495c7e47b8bbd7116a04e1d28f
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# A lazy read that is still one sample: ADR-0011's ceiling-and-page

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
