---
id: HS-S0044
uid: f6e643
type: story
slug: projection-store-passes-the-borrowed-suite
title: SqliteProjectionStore against the suite it did not write
parent: HS-P0012
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0036
blocks:
  - HS-S0045
archetype: capability
slice: sqlite-projection-store
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-18T13:32:07.970Z
links:
  pr: null
  commits:
    - 1afb47b
    - aa35d5e
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# SqliteProjectionStore against the suite it did not write

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
