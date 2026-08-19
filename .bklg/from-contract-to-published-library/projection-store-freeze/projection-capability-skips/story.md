---
id: HS-S0008
uid: 44444c
type: story
slug: projection-capability-skips
title: A declined capability is a reported skip, never a silent absence
parent: HS-P0010
initiative: from-contract-to-published-library
project: projection-store-freeze
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0003
  - HS-S0007
blocks:
  - HS-S0011
  - HS-S0012
archetype: capability
slice: projection-conformance-suite
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-14T17:22:55.067Z
links:
  pr: null
  commits:
    - 7fcb378
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# A declined capability is a reported skip, never a silent absence

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
