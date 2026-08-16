---
id: HS-S0030
uid: 9c8f92
type: story
slug: compile-fail-proof-artefact
title: The compile-fail case, its negative control, and its gate row
parent: HS-P0011
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0029
blocks:
  - HS-S0033
archetype: capability
slice: worked-example-and-proof
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-16T20:45:48.012Z
links:
  pr: null
  commits:
    - 1044a95
    - c4e36c4
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# The compile-fail case, its negative control, and its gate row

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
