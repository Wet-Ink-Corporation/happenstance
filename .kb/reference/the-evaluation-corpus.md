---
id: reference-the-evaluation-corpus
title: "Index: the evaluation corpus, and what each review was for"
kind: reference
status: accepted
authority_tier: note
summary: >-
  `docs/evaluation/` holds the research and adversarial reviews the design rests on —
  including PRESSURE-TEST.md, which refuted a previous plan's headline claim by compiling it,
  and the eight review-* documents that each attacked one axis. Indexed here rather than
  copied, because they are records of a moment and are cited by path.
depends_on: []
related:
  - concept-proof-artefact-vs-green-gate
  - roadmap-the-runway
source_paths:
  - docs/evaluation/README.md
  - docs/evaluation/PRESSURE-TEST.md
  - docs/evaluation/ARCHITECTURAL-EVALUATION.md
last_reviewed: 2026-08-09
---

# The evaluation corpus

## Why it is indexed and not harvested

These are records of what was believed and tested at a moment, and the specification's
clauses cite them **by path**. Copying their content into atoms would create a second copy
that drifts from the one the clauses point at.

## What is in there

- `PRESSURE-TEST.md` — the adversarial pass over the previous runbook. It refuted that
  plan's headline `trait_variant` claim by compiling it, found a re-used ADR number, four
  dropped ledger rows, and a phase whose exit criterion mandated its own outcome.
- `ARCHITECTURAL-EVALUATION.md` — the standing evaluation of the design as a whole.
- `research-*` — the grounding: the DCB specification and existing implementations, the Rust
  API guidelines, the ecosystem, and the composition/integration/stack cruxes.
- `review-*` — eight adversarial reviews, one axis each: correctness, the conformance suite,
  adapter implementability, DX and ergonomics, packaging and semver, docs and ADRs, blind
  spots, and the phase-4 reconciliation.
- `revised-runway.md` — the proposal that got the *method* right and whose bodies did not
  survive the pressure test. Kept because the current plan is its correction.
