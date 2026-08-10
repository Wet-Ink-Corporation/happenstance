---
id: roadmap-the-runway
title: "Roadmap: the phases, the critical path, and what must not be parallelised"
kind: roadmap
status: accepted
authority_tier: note
summary: >-
  Fifteen phases sequenced by BLAST RADIUS rather than by artefact, so the contract is
  settled early rather than in the phase most expensive to revise. Phases 0-5 are done.
  The live plan of record is `.bklg/`; this atom is the shape of it and the constraints on
  reordering, which the backlog cannot express.
depends_on: []
related:
  - playbook-the-phase-protocol
  - reference-e2e-case-catalogue
  - map-the-instrument-portfolio
source_paths:
  - docs/RUNBOOK.md
last_reviewed: 2026-08-09
---

# The runway

## The critical path

```
0 ─▶ 1 ─▶ 2 ─▶ 4 ─▶ 6 ─▶ 7 ─▶ [0.2.0-alpha.1] ─▶ 8 ─▶ 12 ─▶ 13 ─▶ 14

one branch that rejoins the trunk:
    1 ─▶ 3 ─▶ 4          phase 4 is frozen against the instrument phase 3 builds

one that floats — after 4, before 12, otherwise unconstrained:
    4 ─▶ 5               the wire format; nothing between it and 12 reads it

three that never rejoin — off the 0.1 path:
    2, 4    ─▶ 9
    2, 4, 6 ─▶ 10
    6       ─▶ 11
```

Phase 5 is the only slack this plan has: it depends on phase 4 and nothing depends on it
before phase 12, so it is three days that can be spent whenever three days are available.

## What must not be parallelised, and why

- **1 before 2** — the skeletons are written against whatever ADR-0008 decides; writing them
  first means writing them twice.
- **2 before 4** — the skeletons *are* the evidence, and a freeze written before them is a
  freeze written on intention.
- **3 before 4** — every semantic clause phase 4 freezes is paired with a rule, and a rule
  written after the signature it protects is written by someone who already believes the
  signature is right.
- **4 before 8, 9 and 10** — `EventId` and `recorded_at` are columns in migration 1 of every
  store.
- **7 before 8** — the typed layer is the consumer that discovers contract defects, and
  discovering them after the flagship adapter is written is the sequence this plan exists to
  avoid.

## Where the live state lives

`.bklg/` is the plan of record; `redkiln status` reports it. This atom holds the sequencing
argument, which is knowledge rather than work in motion.
