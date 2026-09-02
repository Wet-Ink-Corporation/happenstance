---
id: HS-S0072
uid: 74e22e
type: story
slug: deskeleton-and-package-readiness
title: Neither crate is a skeleton any more
parent: HS-P0014
initiative: from-contract-to-published-library
project: postgres-and-neon-stores
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0060
  - HS-S0066
  - HS-S0070
  - HS-S0071
blocks:
  - HS-S0073
archetype: capability
slice: publish-readiness-and-audit
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T05:10:25.481Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# Neither crate is a skeleton any more

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
