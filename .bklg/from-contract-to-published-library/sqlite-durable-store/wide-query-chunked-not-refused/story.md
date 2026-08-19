---
id: HS-S0039
uid: c387c4
type: story
slug: wide-query-chunked-not-refused
title: A wide Query is chunked and merged, never refused
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0038
blocks:
  - HS-S0040
archetype: capability
slice: durable-event-store
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-18T13:26:10.234Z
links:
  pr: null
  commits:
    - 11596b4
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# A wide Query is chunked and merged, never refused

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
