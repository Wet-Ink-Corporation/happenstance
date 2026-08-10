---
id: reference-e2e-case-catalogue
title: "Index: the 57 end-to-end cases, and which phase makes each writable"
kind: reference
status: accepted
authority_tier: note
summary: >-
  `docs/scenarios/E2E-CASES.md` enumerates the end-to-end cases the library must satisfy.
  Every case is claimed by at least one phase's "cases this makes writable" list, which is
  what stops a case being specified somewhere and scheduled nowhere — a failure the
  adversarial pass found four instances of.
depends_on: []
related:
  - roadmap-the-runway
  - map-the-three-ports
source_paths:
  - docs/scenarios/E2E-CASES.md
  - docs/scenarios/README.md
last_reviewed: 2026-08-09
---

# The end-to-end case catalogue

## What it is

The catalogue of end-to-end cases, each naming the behaviour a user of the library observes
rather than the unit that implements it. Phases claim cases; the claim is what schedules the
work that makes a case writable.

## Why the claiming matters

The adversarial pass over the previous plan found orphans in both directions: cases named by
no phase, and a blocked-case table naming a case that was not blocked while omitting one that
was. A catalogue nobody has to claim from is a catalogue that quietly stops being covered.
