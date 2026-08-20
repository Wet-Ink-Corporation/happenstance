---
id: HS-S0053
uid: 1ea4b3
type: story
slug: durable-object-host-and-fixture
title: A Durable Object host and the CloudflareFixture mounted on it
parent: HS-P0013
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by:
  - HS-S0050
  - HS-S0051
blocks:
  - HS-S0054
  - HS-S0055
archetype: capability
slice: durable-object-conformance-run
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-20T01:24:40.905Z
links:
  pr: null
  commits:
    - 6fc808e5b2aa1d591e53efc1534545bb4c38157f
    - 84d5ab9331ab2acbc6afd1dddd2ad66a907834b8
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# A Durable Object host and the CloudflareFixture mounted on it

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
