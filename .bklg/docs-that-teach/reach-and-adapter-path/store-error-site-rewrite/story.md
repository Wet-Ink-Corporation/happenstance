---
id: HS-S0156
uid: 58f6f9
type: story
slug: store-error-site-rewrite
title: rustc's own E0034 output at the site where it fires
parent: HS-P0023
initiative: docs-that-teach
project: reach-and-adapter-path
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0154
  - HS-S0155
blocks:
  - HS-S0157
archetype: capability
slice: adapter-error-site
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-20T07:08:01.429Z
links:
  pr: null
  commits:
    - 10e99b2
    - f2c7dbe
    - "76e9424"
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# rustc's own E0034 output at the site where it fires

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
