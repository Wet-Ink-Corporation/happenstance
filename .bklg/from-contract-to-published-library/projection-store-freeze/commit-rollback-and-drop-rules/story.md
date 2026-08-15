---
id: HS-S0010
uid: b40991
type: story
slug: commit-rollback-and-drop-rules
title: Commit, rollback and dropped-batch rules, each with the store that fails it
parent: HS-P0010
initiative: from-contract-to-published-library
project: projection-store-freeze
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0009
blocks:
  - HS-S0013
archetype: capability
slice: commit-atomicity-and-mutants
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-15T05:09:16.188Z
links:
  pr: null
  commits:
    - 5d9b4fd
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# Commit, rollback and dropped-batch rules, each with the store that fails it

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
