---
id: reference-experiment-wire-format
title: "Measured: the wire-format probes ADR-0016 cites"
kind: reference
status: accepted
authority_tier: reference
summary: >-
  ADR-0016 cites "probe W1" through "probe W7" roughly forty times as the source of nearly
  every number it states, and when it was written NO probe existed in the repository — they
  were throwaway programs in a lost scratchpad. This directory is those programs, kept, so
  `cargo test -- --nocapture` regenerates every cited number from source.
depends_on: []
related:
  - adr-0016-the-wire-format
  - concept-proof-artefact-vs-green-gate
source_paths:
  - docs/experiments/wire-format/README.md
last_reviewed: 2026-08-09
---

# The wire-format measurements

## What this is

The real programs behind ADR-0016's probes W1–W7, kept rather than described. The seed for
the one probe that needed one is written into the file rather than described in prose.

## Why it exists — and the lesson worth carrying

"Measured, probe W5" was a **label, not a citation**. The probes were run once, in throwaway
crates under a session scratchpad, and the scratchpad was deleted. Six months later the ADR's
numbers would have been unreproducible in exactly the way this repository warns about
elsewhere: the *form* of reproducibility rather than the thing itself.

The generalisation: a citation that cannot be re-run is a claim wearing a citation's clothes.

## Standing

Not a workspace member, not a `cargo xtask ci` step. `docs/experiments/` is "measurements,
reproducible, and not in the gate" — both halves load-bearing.
