---
id: HS-S0106
uid: 5ea6f3
type: story
slug: message-set-on-the-envelope
title: The message set instantiates Envelope<T>
parent: HS-P0017
initiative: from-contract-to-published-library
project: replication-identity-and-ingest
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0099
  - HS-S0105
blocks:
  - HS-S0107
  - HS-S0111
archetype: capability
slice: wire-message-set-and-round-trip
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-13T14:12:01.928Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The message set instantiates Envelope<T>

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
