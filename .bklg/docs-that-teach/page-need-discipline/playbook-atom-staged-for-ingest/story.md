---
id: HS-S0153
uid: 8dd29a
type: story
slug: playbook-atom-staged-for-ingest
title: The playbook atom staged under .kb/_intake/, never hand-authored into .kb/
parent: HS-P0021
initiative: docs-that-teach
project: page-need-discipline
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0148
  - HS-S0149
  - HS-S0151
blocks: []
archetype: capability
slice: binding-beyond-this-project
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-19T00:26:57.198Z
links:
  pr: null
  commits:
    - dc58e80
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The playbook atom staged under .kb/_intake/, never hand-authored into .kb/

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
