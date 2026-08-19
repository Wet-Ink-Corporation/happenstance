---
id: HS-S0108
uid: e1fe2c
type: story
slug: adr-0003-provisional-lift
title: ADR-0003 loses provisional by a new atom, never by an edit
parent: HS-P0017
initiative: from-contract-to-published-library
project: replication-identity-and-ingest
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0107
blocks: []
archetype: foundation
slice: wire-message-set-and-round-trip
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T14:12:08.016Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# ADR-0003 loses provisional by a new atom, never by an edit

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
