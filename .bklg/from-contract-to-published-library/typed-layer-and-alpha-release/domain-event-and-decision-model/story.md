---
id: HS-S0020
uid: e409fc
type: story
slug: domain-event-and-decision-model
title: DomainEvent and DecisionModel, mounted at the crate root
parent: HS-P0011
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0018
blocks:
  - HS-S0021
  - HS-S0023
archetype: capability
slice: typed-vocabulary
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-16T20:20:46.842Z
links:
  pr: null
  commits:
    - 996853f
    - 3fde7d1
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# DomainEvent and DecisionModel, mounted at the crate root

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
