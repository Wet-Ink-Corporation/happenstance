---
id: HS-S0024
uid: 9f617e
type: story
slug: misbehaving-testkit-stores
title: FaultyStore<S> and GappyMemoryStore in the testkit
parent: HS-P0011
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: ready
process: story
stage: plan
automation: HITL
severity: null
blocked_by: []
blocks:
  - HS-S0025
archetype: capability
slice: testing-surface
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-16T19:59:08.067Z
links:
  pr: null
  commits:
    - 6c59c46
    - 0ea1ea0
    - d95c760
    - b77cffb
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# FaultyStore<S> and GappyMemoryStore in the testkit

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.

## Inbound handoffs

Work routed to this story because it owns `standards/` and `crates/happenstance-testkit/`, recorded
so it is neither done twice nor lost.

- **Three stale constitution citations — routed here, and already landed.** `cargo xtask
  lint-constitution` (a REQUIRED step of `cargo xtask ci --fast`) failed on
  `standards/rust/60-what-a-test-must-prove.md:68-69` and `:163`, whose evidence lines pointed at
  `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:454`, `:330` and `:296`. The
  anchors had moved twelve lines when `90421d0 fix(typed-layer-and-alpha-release): repair
  contaminated baseline test` edited that harness. The **typed-vocabulary** slice (M2) found it
  because `ci --fast` is its declared merge bar, could not attribute it to itself, and landed the
  three-line repair as its own checkpoint carrying `Story:
  typed-layer-and-alpha-release/misbehaving-testkit-stores` rather than folding it into a slice
  commit. The citations now read `:466`, `:342` and `:308`; the anchor text is unchanged, and
  `cargo xtask lint-constitution` reports *27 atoms, all consistent*. **Nothing is left to do here
  for it** — this entry exists so the drift is not re-diagnosed from scratch when this story runs.
