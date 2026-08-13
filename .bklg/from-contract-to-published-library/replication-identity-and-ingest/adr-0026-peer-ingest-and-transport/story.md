---
id: HS-S0098
uid: 9fb4d1
type: story
slug: adr-0026-peer-ingest-and-transport
title: "ADR-0026: what a peer is, what the port may assume, what ingest promises"
parent: HS-P0017
initiative: from-contract-to-published-library
project: replication-identity-and-ingest
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by: []
blocks:
  - HS-S0099
  - HS-S0100
  - HS-S0101
  - HS-S0112
archetype: foundation
slice: decisions-of-record
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T14:11:52.787Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# ADR-0026: what a peer is, what the port may assume, what ingest promises

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
