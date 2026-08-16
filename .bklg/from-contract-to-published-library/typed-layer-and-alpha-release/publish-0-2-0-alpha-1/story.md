---
id: HS-S0033
uid: 406b2b
type: story
slug: publish-0-2-0-alpha-1
title: 0.2.0-alpha.1 on the registry, with its churn mitigations
parent: HS-P0011
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0027
  - HS-S0030
  - HS-S0031
  - HS-S0032
blocks: []
archetype: capability
slice: alpha-release
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-16T23:11:27.443Z
links:
  pr: null
  commits:
    - 448e1ac
  kb: []
gate_open: true
schema: 1
process_rev: b611dd09
---
# 0.2.0-alpha.1 on the registry, with its churn mitigations

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.

## Inbound handoffs

Findings routed here by an earlier slice rather than fixed there. Each names the exact line and the
AC of this story's `spec.md` that already covers it; none of them widens this story's scope.

- **`crates/happenstance/README.md:6` contradicts the crate root, and the contradiction ships.**
  The line reads *"**Status: early, and this crate is currently a facade.** It re-exports
  [`happenstance-core`](https://crates.io/crates/happenstance-core) and adds nothing yet."* As of
  the **typed-vocabulary** slice (M2 — `domain-event-and-decision-model` +
  `decision-model-composition`) that is false: `crates/happenstance/src/lib.rs` exports
  `DomainEvent`, `DecisionModel`, `Boundary`, `Codec` and `CodecError`, and its module doc opens
  with a program that uses them. `crates/happenstance/src/lib.rs:10`
  (`#![cfg_attr(doctest, doc = include_str!("../README.md"))]`) compiles this README into the
  doctest set, so the page is both published-facing and gated — and the two surfaces currently
  disagree about whether the typed layer exists.

  Already covered by **AC-003** of this story's `spec.md`, which requires the blockquote at
  `crates/happenstance/README.md:6-11` **deleted** rather than annotated and the word *facade*
  absent from the page (`rg -n "facade" crates/happenstance/README.md` returning nothing). Logged
  here so the alpha is not cut with the two pages disagreeing; `crate-readme` is M5-M7's surface
  and was explicitly outside M2's PR globs, so M2 could not and did not touch it.
