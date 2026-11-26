---
id: kb-decision-0120
title: Phase 22, the documentation site, does not gate 1.0.0
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0120
reversibility: high
phase: 21
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-25 by the v1 AFK run for the owner (#209, second half). Phase 21's dependency
  row is 13, 14, 16, 17, 17b, 18, 20, and phase 22 (the site) is in neither the row nor the prose.
  The site is live and deployed from main, and lacks only its exit evidence; the 1.0 soak's
  conditions are about docs.rs, the registry and an outside reader, none of which needs the site.
  Recommended: phase 22 does not gate 1.0.0; its exit is closed when its evidence is in, before or
  after. The flip is the owner deciding the outside reader must start at the site.
depends_on:
  - kb-decision-0066
related:
  - kb-decision-0088
source_paths:
  - runbook/README.md
  - runbook/phases/21-one-point-oh.md
  - runbook/phases/22-docs-site.md
last_reviewed: 2026-11-25
---

# Phase 22, the documentation site, does not gate `1.0.0`

**Proposed 2026-11-25** by the unattended v1 run, for the owner (#209). Nothing below has been
applied. The two-way half of #209, phase 21's prose naming 17b as its row does, is its own PR.

## Context

D-1 put sync inside 1.0, and phase 21's status row enforces it: 13, 14, 16, 17, 17b, 18, 20
(`runbook/README.md`). Phase 22, the documentation site, is in neither that row nor phase 21's
prose. It is live — deployed from `main` by `pages.yml` — and lacks only its exit evidence: the
broken-link criterion (watched on 2026-11-25: a broken anchor failed the build, a link to a missing
page did not, and #547 fixes the second), and the owner's ticks.

ADR-0066's soak conditions are: all nine crates render on docs.rs from the registry; two examples
build against the registry rc; the registry semver baseline is clean; the outside-reader pass has
run against the rc; no defect the rc surfaced needs an API change. None names the site. Phase 20's
friction log (HS-P0024, #174) does start its reader at the site, so the reader exercises it either
way.

## Options

- **A. Phase 22 does not gate 1.0 (recommended).** The site is a presentation of `docs/`, which the
  gate already compiles and phase 20 already gates. Its exit closes when its evidence is in.
- B. Phase 22 gates 1.0: add 22 to phase 21's row, its prose and epic #68's blockers. Costs little
  if the evidence lands soon, and makes a presentation layer a release blocker.

## Decision (proposed)

A. On acceptance: phase 21's row is unchanged, #209 records the decision and closes, and phase 22
keeps its own row and exit criteria.

## What would flip it

The owner deciding the outside reader (#166, #203) must start at the site rather than on docs.rs,
which would make the site part of what 1.0 promises a newcomer.

## Door

Two-way in effect, but it moves the plan of record's release gate, which is the owner's.
