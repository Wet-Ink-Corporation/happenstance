---
id: HS-S0007
uid: d20a9f
type: story
slug: projection-suite-entry-point
title: projection_store_conformance! — one enumeration, one test per rule
parent: HS-P0010
initiative: from-contract-to-published-library
project: projection-store-freeze
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0004
  - HS-S0005
  - HS-S0006
blocks:
  - HS-S0008
  - HS-S0009
archetype: capability
slice: projection-conformance-suite
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-14T16:58:05.955Z
links:
  pr: null
  commits:
    - 79df6b7
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# projection_store_conformance! — one enumeration, one test per rule

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
