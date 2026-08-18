---
id: HS-S0040
uid: 3409c8
type: story
slug: sqlite-fixture-and-whole-suite
title: SqliteFixture, and the first green conformance run against a file
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0037
  - HS-S0039
blocks:
  - HS-S0041
  - HS-S0043
archetype: capability
slice: durable-event-store
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-18T13:27:01.060Z
links:
  pr: null
  commits:
    - 41a2064
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# SqliteFixture, and the first green conformance run against a file

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
