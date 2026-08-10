---
id: reference-experiment-rustc-ice-gat-foreign-trait
title: "Measured: a rustc ICE this architecture reaches by construction"
kind: reference
status: accepted
authority_tier: reference
summary: >-
  A projection store that BORROWS its database rather than owning it does not compile —
  correctly — and rustc then crashes while writing the error message. The bug is entirely in
  the path that EXPLAINS a region error, never in the one that detects it, and it fires only
  when the trait is foreign.
depends_on: []
related:
  - concept-the-batch-shape-axis
  - question-projection-store-port-is-provisional
source_paths:
  - docs/experiments/rustc-ice-gat-foreign-trait/README.md
last_reviewed: 2026-08-09
---

# A rustc ICE reached by construction

## What happens

A store type that borrows its database carries a lifetime and does not outlive the one the
trait method requires. rustc works that out — correctly — and then panics while producing
the diagnostic:

```text
thread 'rustc' panicked at compiler\rustc_trait_selection\src\errors\note_and_explain.rs:27:22:
DefId::expect_local: `DefId(...::Store::commit)` isn't local
```

## Why it happens

The diagnostics code tries to point at the trait method to explain *where* the lifetime
obligation came from, and assumes that item is in the crate being compiled. When the trait is
**foreign** — which it always is for an adapter implementing this workspace's port — the
assumption is false and rustc panics on the spot.

## Why it matters here

It is not an exotic edge case for this architecture: it is what an adapter author meets the
first time they try the borrowed batch shape. The error they need is the one rustc crashes
while writing, so the failure presents as a compiler bug rather than as the design constraint
it actually is. That is a cold-start cost the port's documentation has to pay down.
