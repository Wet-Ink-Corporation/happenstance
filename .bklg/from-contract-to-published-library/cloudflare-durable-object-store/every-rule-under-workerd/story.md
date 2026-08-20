---
id: HS-S0054
uid: b90c7d
type: story
slug: every-rule-under-workerd
title: Every event-store conformance rule executed on the target, in the same gate run
parent: HS-P0013
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0048
  - HS-S0053
blocks:
  - HS-S0055
  - HS-S0056
  - HS-S0057
  - HS-S0058
  - HS-S0059
archetype: capability
slice: durable-object-conformance-run
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-20T20:40:03.162Z
links:
  pr: null
  commits:
    - 440bbacab4be8746b5eb1f4a19b6020f8efe8fb9
    - 84d5ab9331ab2acbc6afd1dddd2ad66a907834b8
    - 440bbac
    - 1c23740
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# Every event-store conformance rule executed on the target, in the same gate run

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
